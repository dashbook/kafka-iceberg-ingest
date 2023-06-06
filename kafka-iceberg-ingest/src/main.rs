use std::{env, sync::Arc};

use apache_avro::from_avro_datum;
use futures::stream::TryStreamExt;
use kafka_iceberg_ingest::schema::{get_key_schema, get_value_schema};
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

    let key_schema = Arc::new(
        get_key_schema(&schema_registry_host, &group_id, &topic)
            .await
            .expect("Failed to get schema."),
    );

    let value_schema = Arc::new(
        get_value_schema(&schema_registry_host, &group_id, &topic)
            .await
            .expect("Failed to get schema."),
    );

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", &broker)
        .create()
        .expect("Failed to create Kafka consumer.");

    consumer
        .subscribe(&[&topic])
        .expect("Failed to subscribe to ${topic}");

    let result = consumer
        .stream()
        .and_then(|message| {
            let key_schema = key_schema.clone();
            let value_schema = value_schema.clone();
            async move {
                dbg!(from_avro_datum(&key_schema, &mut message.key().unwrap(), None).unwrap());
                match message.payload() {
                    Some(mut bytes) => {
                        from_avro_datum(&value_schema, &mut bytes, None).map_err(|err| {
                            println!("{}", err);
                            KafkaError::MessageConsumption(RDKafkaErrorCode::BadMessage)
                        })
                    }
                    None => Err(KafkaError::MessageConsumption(RDKafkaErrorCode::BadMessage)),
                }
            }
        })
        .try_collect::<Vec<_>>()
        .await
        .expect("Failed to read message stream.");

    dbg!(&result);

    // let _result = consumer
    //     .stream()
    //     .try_chunks(arrow_batch_size)
    //     .then(|messages| async move {
    //         let messages = messages.map_err(anyhow::Error::msg)?;
    //         let size = messages[0].payload_len() * arrow_batch_size;
    //         let bytes = messages
    //             .iter()
    //             .fold(Vec::with_capacity(size), |mut acc, x| {
    //                 if let Some(bytes) = x.payload() {
    //                     acc.extend_from_slice(bytes);
    //                 }
    //                 acc
    //             });
    //         let _reader = Reader::new(bytes.as_slice())?;
    //         Ok::<_, anyhow::Error>(1)
    //     })
    //     .for_each_concurrent(None, |_record_batch| async move {})
    //     .await;
}
