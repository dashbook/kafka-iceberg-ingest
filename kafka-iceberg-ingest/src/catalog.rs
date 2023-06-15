use std::sync::Arc;

use iceberg_catalog_rest_metadata_file::{
    apis::configuration::Configuration, catalog::RestCatalog,
};
use iceberg_rust::{catalog::Catalog, object_store::aws::AmazonS3Builder};

pub fn get_catalog(
    region: &str,
    bucket: &str,
    catalog_url: &str,
    authorization_header: &str,
) -> Result<Arc<dyn Catalog>, anyhow::Error> {
    let mut configuration = Configuration::new();
    configuration.base_path = catalog_url.to_string();
    configuration.bearer_access_token = Some(authorization_header.to_string());

    let object_store = Arc::new(
        AmazonS3Builder::new()
            .with_region(region)
            .with_bucket_name(bucket)
            .build()?,
    );

    Ok(Arc::new(RestCatalog::new(
        "default",
        configuration,
        object_store,
    )))
}
