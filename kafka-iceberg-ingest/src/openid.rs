use openidconnect::{
    core::{
        CoreClient, CoreGenderClaim, CoreJsonWebKeyType, CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreTokenType,
    },
    reqwest::async_http_client,
    ClientId, DeviceAuthorizationResponse, DeviceAuthorizationUrl, EmptyAdditionalClaims,
    EmptyExtraDeviceAuthorizationFields, EmptyExtraTokenFields, IdTokenFields, IssuerUrl,
    StandardTokenResponse,
};

pub async fn get_tokens(
    issuer_url: &str,
    client_id: &str,
) -> Result<
    StandardTokenResponse<
        IdTokenFields<
            EmptyAdditionalClaims,
            EmptyExtraTokenFields,
            CoreGenderClaim,
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
            CoreJsonWebKeyType,
        >,
        CoreTokenType,
    >,
    anyhow::Error,
> {
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(issuer_url.to_string())?,
        async_http_client,
    )
    .await?;
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(client_id.to_string()),
        None,
    )
    .set_device_authorization_uri(DeviceAuthorizationUrl::new(
        issuer_url.to_string() + "/oauth/v2/device_authorization",
    )?);
    let details: DeviceAuthorizationResponse<EmptyExtraDeviceAuthorizationFields> = client
        .exchange_device_code()?
        .request_async(async_http_client)
        .await?;

    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        details.verification_uri().to_string(),
        details.user_code().secret().to_string()
    );
    client
        .exchange_device_access_token(&details)
        .request_async(async_http_client, tokio::time::sleep, None)
        .await
        .map_err(anyhow::Error::msg)
}
