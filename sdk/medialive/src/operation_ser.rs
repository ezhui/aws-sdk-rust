// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_batch_delete(
    input: &crate::input::BatchDeleteInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_batch_delete_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_batch_start(
    input: &crate::input::BatchStartInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_batch_start_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_batch_stop(
    input: &crate::input::BatchStopInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_batch_stop_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_batch_update_schedule(
    input: &crate::input::BatchUpdateScheduleInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_batch_update_schedule_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_channel(
    input: &crate::input::CreateChannelInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_channel_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_input(
    input: &crate::input::CreateInputInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_input_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_input_security_group(
    input: &crate::input::CreateInputSecurityGroupInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_input_security_group_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_multiplex(
    input: &crate::input::CreateMultiplexInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_multiplex_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_multiplex_program(
    input: &crate::input::CreateMultiplexProgramInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_multiplex_program_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_partner_input(
    input: &crate::input::CreatePartnerInputInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_partner_input_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_tags(
    input: &crate::input::CreateTagsInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_create_tags_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_purchase_offering(
    input: &crate::input::PurchaseOfferingInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_purchase_offering_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_transfer_input_device(
    input: &crate::input::TransferInputDeviceInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_transfer_input_device_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_channel(
    input: &crate::input::UpdateChannelInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_channel_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_channel_class(
    input: &crate::input::UpdateChannelClassInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_channel_class_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_input(
    input: &crate::input::UpdateInputInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_input_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_input_device(
    input: &crate::input::UpdateInputDeviceInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_input_device_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_input_security_group(
    input: &crate::input::UpdateInputSecurityGroupInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_input_security_group_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_multiplex(
    input: &crate::input::UpdateMultiplexInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_multiplex_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_multiplex_program(
    input: &crate::input::UpdateMultiplexProgramInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_multiplex_program_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_update_reservation(
    input: &crate::input::UpdateReservationInput,
) -> Result<smithy_http::body::SdkBody, serde_json::error::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_update_reservation_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}