// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<smithy_types::Error, serde_json::Error> {
    let body =
        serde_json::from_slice(response.body().as_ref()).unwrap_or_else(|_| serde_json::json!({}));
    Ok(crate::aws_json_errors::parse_generic_error(
        &response, &body,
    ))
}

pub fn internal_service_exception(
    inp: &[u8],
    mut builder: crate::error::internal_service_error::Builder,
) -> Result<crate::error::internal_service_error::Builder, serde_json::Error> {
    let parsed_body: crate::error::InternalServiceError = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_message(parsed_body.message);
    Ok(builder)
}

pub fn get_device_registration_deser_operation(
    inp: &[u8],
    mut builder: crate::output::get_device_registration_output::Builder,
) -> Result<crate::output::get_device_registration_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GetDeviceRegistrationOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_device_registration(parsed_body.device_registration);
    builder = builder.set_cache_ttl(parsed_body.cache_ttl);
    Ok(builder)
}