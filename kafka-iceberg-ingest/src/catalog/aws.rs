use anyhow::anyhow;
use aws_config::SdkConfig;
use aws_sdk_sts::{config::Region, Client};
use iceberg_rust::object_store::{aws::AmazonS3Builder, ObjectStore};
use std::sync::Arc;

pub async fn get_s3(
    region: &str,
    bucket: &str,
    token: &str,
    role: &str,
) -> Result<Arc<dyn ObjectStore>, anyhow::Error> {
    let sdk_config = SdkConfig::builder()
        .region(Some(Region::new(region.to_owned())))
        .build();
    let sts_client = Client::new(&sdk_config);
    // let credentials = sts_client
    //     .assume_role_with_web_identity()
    //     .set_role_arn(Some(role.to_string()))
    //     .set_role_session_name(Some(token[0..63].to_string()))
    //     .set_web_identity_token(Some(token.to_string()))
    //     .send()
    //     .await?;
    // let credentials = credentials
    //     .credentials()
    //     .ok_or(anyhow!("Failed to get aws credentials."))?;
    Ok(Arc::new(
        AmazonS3Builder::new()
            .with_region(region)
            .with_bucket_name(bucket)
            // .with_access_key_id(
            //     credentials
            //         .access_key_id()
            //         .ok_or(anyhow!("Failed to get access_key_id from aws credentials."))?,
            // )
            // .with_secret_access_key(credentials.secret_access_key().ok_or(anyhow!(
            //     "Failed to get secret access key from aws credentials."
            // ))?)
            .build()
            .map_err(anyhow::Error::msg)?,
    ))
}
