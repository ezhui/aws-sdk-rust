// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_get_introspection_schema_get_introspection_schema_output_schema(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<smithy_types::Blob>,
    crate::error::GetIntrospectionSchemaError,
> {
    (!body.is_empty())
        .then(|| Ok(smithy_types::Blob::new(body)))
        .transpose()
}