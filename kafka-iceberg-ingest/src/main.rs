use std::{env, sync::Arc};

use apache_avro::from_avro_datum;
use futures::stream::{self, StreamExt, TryStreamExt};
use kafka_iceberg_ingest::schema::{debezium_schema, get_value_schema};
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    error::KafkaError,
    types::RDKafkaErrorCode,
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

    let value_schema = Arc::new(
        get_value_schema(&schema_registry_host, &group_id, &topic)
            .await
            .expect("Failed to get schema."),
    );

    let table_schema = if debezium {
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

    stream::iter(partitions.iter())
        .for_each_concurrent(None, |partition| {
            let consumer = consumer.clone();
            let value_schema = value_schema.clone();
            let topic = topic.clone();
            async move {
                let temp = consumer
                    .split_partition_queue(&topic, *partition)
                    .expect("Failed to get stream for partition {partition}")
                    .stream()
                    .and_then(|message| {
                        let value_schema = value_schema.clone();
                        async move {
                            match message.payload() {
                                Some(bytes) => {
                                    from_avro_datum(&value_schema, &mut bytes.split_at(9).1, None)
                                        .map_err(|err| {
                                            println!("{}", err);
                                            KafkaError::MessageConsumption(
                                                RDKafkaErrorCode::BadMessage,
                                            )
                                        })
                                }
                                None => Err(KafkaError::MessageConsumption(
                                    RDKafkaErrorCode::BadMessage,
                                )),
                            }
                        }
                    })
                    .try_chunks(arrow_batch_size)
                    .try_collect::<Vec<_>>()
                    .await
                    .unwrap();
            }
        })
        .await;
}
