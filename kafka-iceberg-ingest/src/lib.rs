use anyhow::anyhow;
use apache_avro::types::Value;

pub mod arrow;
pub mod schema;

pub fn value_to_record(
    value: Value,
    debezium: Option<usize>,
) -> Result<Vec<(String, Value)>, anyhow::Error> {
    if let Value::Record(mut record) = value {
        if let Some(index) = debezium {
            if let Value::Record(record) = record.remove(index).1 {
                Ok(record)
            } else {
                Err(anyhow!("Failed to coerce avro value to record."))
            }
        } else {
            Ok(record)
        }
    } else {
        Err(anyhow!("Failed to coerce avro value to record."))
    }
}
