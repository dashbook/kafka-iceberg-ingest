use std::{
    env,
    pin::Pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::anyhow;
use apache_avro::{from_avro_datum, types::Value, Schema};
use arrow::{datatypes::Schema as ArrowSchema, error::ArrowError};
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    stream::{self, StreamExt, TryStreamExt},
    SinkExt, Stream,
};
use iceberg_rust::{arrow::write::write_parquet_files, catalog::relation::Relation};
use kafka_iceberg_ingest::{
    arrow::avro_to_arrow_batch,
    catalog::get_catalog,
    schema::{debezium_schema, get_value_schema},
    value_to_record,
    zipstream::ZipStream,
};
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};

#[tokio::main]
pub async fn main() {
    let topic = env::var("TOPIC").expect("No topic is specified.");
    let broker = env::var("BROKER").expect("No broker is specified.");
    let group_id = env::var("GROUP_ID").unwrap_or("default".to_string());
    let arrow_batch_size = env::var("ARROW_BATCH_SIZE")
        .unwrap_or("8192".to_string())
        .parse::<usize>()
        .expect("Failed to parse arrow batch size.");
    let schema_registry_host =
        env::var("SCHEMA_REGISTRY_HOST").unwrap_or("http://localhost".to_string());
    let debezium = env::var("DEBEZIUM")
        .unwrap_or("true".to_string())
        .to_lowercase()
        .parse::<bool>()
        .expect("Failed to parse arrow batch size.");

    let latency = env::var("LATENCY")
        .unwrap_or("3600".to_string())
        .parse::<u64>()
        .expect("Failed to parse arrow batch size.");

    let region = env::var("REGION").expect("Region url required.");

    let bucket = env::var("BUCKET").expect("Bucket url required.");

    let catalog_url = env::var("CATALOG_URL").expect("Catalog url required.");

    let authorization_header =
        env::var("AUTHORIZATION_HEADER").expect("Authorization header required.");

    let namespace = env::var("NAMESPACE").unwrap_or("public".to_string());

    let catalog = get_catalog(&region, &bucket, &catalog_url, &authorization_header)
        .expect("Failed to get catalog.");

    let value_schema = Arc::new(
        get_value_schema(&schema_registry_host, &group_id, &topic)
            .await
            .expect("Failed to get schema."),
    );

    let kafka_schema = if debezium {
        Arc::new(debezium_schema(&value_schema).expect("Failed to get table schema."))
    } else {
        value_schema.clone()
    };

    let consumer: Arc<StreamConsumer> = Arc::new(
        ClientConfig::new()
            .set("group.id", &group_id)
            .set("bootstrap.servers", &broker)
            .create()
            .expect("Failed to create Kafka consumer."),
    );

    consumer
        .subscribe(&[&topic])
        .expect("Failed to subscribe to ${topic}");

    let partitions = consumer
        .subscription()
        .expect("Consumer has no subscription")
        .elements()
        .into_iter()
        .map(|x| x.partition())
        .collect::<Vec<_>>();

    let (partition_senders, partition_recievers): (
        Vec<(
            i32,
            UnboundedSender<
                Pin<Box<dyn Stream<Item = Result<Vec<(String, Value)>, anyhow::Error>>>>,
            >,
        )>,
        Vec<
            UnboundedReceiver<
                Pin<Box<dyn Stream<Item = Result<Vec<(String, Value)>, anyhow::Error>>>>,
            >,
        >,
    ) = partitions
        .iter()
        .map(|i| {
            let (sender, reciever) = unbounded();
            ((*i, sender), reciever)
        })
        .unzip();

    stream::iter(partition_senders.into_iter())
        .for_each_concurrent(None, |(partition, mut partition_sender)| {
            let consumer = consumer.clone();
            let value_schema = value_schema.clone();
            let topic = topic.clone();
            async move {
                let last_sync = Arc::new(AtomicU64::new(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Failed to get current time.")
                        .as_secs(),
                ));

                let (sender, reciever) = unbounded();
                partition_sender
                    .send(Box::pin(reciever))
                    .await
                    .expect("Failed to send message to partition stream.");
                let partition = consumer
                    .split_partition_queue(&topic, partition)
                    .expect("Failed to get stream for partition {partition}");
                partition
                    .stream()
                    .map_err(|_| anyhow!("Failed to create Kafka message stream."))
                    .and_then(|message| {
                        let value_schema = value_schema.clone();
                        async move {
                            match message.payload() {
                                Some(bytes) => {
                                    let value = from_avro_datum(
                                        &value_schema,
                                        &mut bytes.split_at(9).1,
                                        None,
                                    )?;
                                    let record = if debezium {
                                        let index = if let &Schema::Record {
                                            name: _name,
                                            aliases: _aliases,
                                            doc: _doc,
                                            fields: _fields,
                                            lookup,
                                        } = &value_schema.as_ref()
                                        {
                                            lookup.get("after").cloned()
                                        } else {
                                            None
                                        };
                                        value_to_record(value, index)?
                                    } else {
                                        value_to_record(value, None)?
                                    };
                                    Ok(record)
                                }
                                None => Err(anyhow!("Message contains no payload.")),
                            }
                        }
                    })
                    .fold(Ok::<_, anyhow::Error>(sender), |sender, x| {
                        let mut partition_sender = partition_sender.clone();
                        let last_sync = last_sync.clone();
                        async move {
                            let update = last_sync.fetch_update(
                                Ordering::Release,
                                Ordering::Acquire,
                                |last| {
                                    let current = SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .expect("Failed to get current time.")
                                        .as_secs();
                                    if last + latency < current {
                                        Some(current)
                                    } else {
                                        None
                                    }
                                },
                            );
                            let mut sender = if update.is_ok() {
                                sender?.close_channel();
                                let (new_sender, new_reciever) = unbounded();
                                partition_sender.send(Box::pin(new_reciever)).await?;
                                Ok(new_sender)
                            } else {
                                sender
                            }?;
                            sender.send(x).await?;
                            Ok(sender)
                        }
                    })
                    .await
                    .expect("Failed to split input stream into multiple streams.")
                    .close_channel();
            }
        })
        .await;

    ZipStream::new(partition_recievers)
        .for_each_concurrent(None, |streams| {
            let catalog = catalog.clone();
            let namespace = namespace.clone();
            let topic = topic.clone();
            let kafka_schema = kafka_schema.clone();
            async move {
                let mut table = match catalog
                    .load_table(
                        &(namespace + "." + &topic)
                            .as_str()
                            .try_into()
                            .expect("Couldn't parse table identifier."),
                    )
                    .await
                    .expect("Failed to load table.")
                {
                    Relation::Table(table) => table,
                    _ => panic!("Can only insert into table."),
                };
                let metadata = table.metadata();
                let location = metadata.location();
                let schema: Arc<ArrowSchema> = Arc::new(
                    metadata
                        .current_schema()
                        .try_into()
                        .expect("Failed to convert schema to arrow."),
                );
                let object_store = table.object_store();

                let files = Box::pin(stream::iter(streams.into_iter()).then(|stream| {
                    let kafka_schema = kafka_schema.clone();
                    let object_store = object_store.clone();
                    let schema = schema.clone();
                    async move {
                        let batches = stream
                            .try_chunks(arrow_batch_size)
                            .map_err(|_| anyhow!("Failed to create arrow record batches."))
                            .and_then(|values| {
                                let kafka_schema = kafka_schema.clone();
                                async move {
                                    avro_to_arrow_batch(
                                        &kafka_schema,
                                        values.iter().collect::<Vec<_>>().as_slice(),
                                    )
                                }
                            })
                            .map_err(|_| {
                                ArrowError::ExternalError(
                                    anyhow!("Failed to create arrow record batches.").into(),
                                )
                            });
                        write_parquet_files(location, &schema, batches, object_store).await
                    }
                }))
                .try_concat()
                .await
                .expect("Failed to write parquet files.");

                table
                    .new_transaction()
                    .append(files)
                    .commit()
                    .await
                    .expect("Failed to perform table update.")
            }
        })
        .await;
}
