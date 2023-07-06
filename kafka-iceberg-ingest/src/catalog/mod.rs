use std::sync::Arc;
// #[cfg(feature = "aws")]
pub mod aws;

use anyhow::anyhow;
use iceberg_catalog_rest_metadata_file::{
    apis::configuration::Configuration, catalog::RestCatalog,
};
use iceberg_rust::{catalog::Catalog, object_store::ObjectStore};

pub async fn get_catalog(
    region: &str,
    bucket: &str,
    catalog_url: &str,
    access_token: &str,
    role: &str,
) -> Result<Arc<dyn Catalog>, anyhow::Error> {
    let mut configuration = Configuration::new();
    configuration.base_path = catalog_url.to_string();
    configuration.bearer_access_token = Some(access_token.to_string());

    let mut object_store = Err::<Arc<dyn ObjectStore>, _>(anyhow!(
        "Couldn't identify the cloud provider for object storage."
    ));
    // #[cfg(feature = "aws")]
    object_store = aws::get_s3(region, bucket, access_token, role).await;

    let catalog_name = catalog_url.to_owned();
    let catalog_name = catalog_name
        .trim_end_matches("/")
        .split("/")
        .last()
        .ok_or(anyhow!("Failed to get catalog name from url"))?;

    Ok(Arc::new(RestCatalog::new(
        catalog_name,
        configuration,
        object_store?,
    )))
}
