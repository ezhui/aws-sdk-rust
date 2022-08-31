// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<aws_smithy_types::Error, aws_smithy_json::deserialize::Error> {
    crate::json_errors::parse_generic_error(response.body(), response.headers())
}

pub fn deser_structure_crate_error_access_denied_exception_json_err(
    value: &[u8],
    mut builder: crate::error::access_denied_exception::Builder,
) -> Result<crate::error::access_denied_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_conflict_exception_json_err(
    value: &[u8],
    mut builder: crate::error::conflict_exception::Builder,
) -> Result<crate::error::conflict_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_internal_server_exception_json_err(
    value: &[u8],
    mut builder: crate::error::internal_server_exception::Builder,
) -> Result<crate::error::internal_server_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_service_quota_exceeded_exception_json_err(
    value: &[u8],
    mut builder: crate::error::service_quota_exceeded_exception::Builder,
) -> Result<
    crate::error::service_quota_exceeded_exception::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_validation_exception_json_err(
    value: &[u8],
    mut builder: crate::error::validation_exception::Builder,
) -> Result<crate::error::validation_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_resource_not_found_exception_json_err(
    value: &[u8],
    mut builder: crate::error::resource_not_found_exception::Builder,
) -> Result<crate::error::resource_not_found_exception::Builder, aws_smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_get_account_alias(
    value: &[u8],
    mut builder: crate::output::get_account_alias_output::Builder,
) -> Result<crate::output::get_account_alias_output::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "accountAlias" => {
                        builder = builder.set_account_alias(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_list_slack_channel_configurations(
    value: &[u8],
    mut builder: crate::output::list_slack_channel_configurations_output::Builder,
) -> Result<
    crate::output::list_slack_channel_configurations_output::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "nextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "slackChannelConfigurations" => {
                        builder = builder.set_slack_channel_configurations(
                            crate::json_deser::deser_list_com_amazonaws_supportapp_slack_channel_configuration_list(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_list_slack_workspace_configurations(
    value: &[u8],
    mut builder: crate::output::list_slack_workspace_configurations_output::Builder,
) -> Result<
    crate::output::list_slack_workspace_configurations_output::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "nextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "slackWorkspaceConfigurations" => {
                        builder = builder.set_slack_workspace_configurations(
                            crate::json_deser::deser_list_com_amazonaws_supportapp_slack_workspace_configuration_list(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_update_slack_channel_configuration(
    value: &[u8],
    mut builder: crate::output::update_slack_channel_configuration_output::Builder,
) -> Result<
    crate::output::update_slack_channel_configuration_output::Builder,
    aws_smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "channelId" => {
                        builder = builder.set_channel_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "channelName" => {
                        builder = builder.set_channel_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "channelRoleArn" => {
                        builder = builder.set_channel_role_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "notifyOnAddCorrespondenceToCase" => {
                        builder = builder.set_notify_on_add_correspondence_to_case(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(
                                tokens.next(),
                            )?,
                        );
                    }
                    "notifyOnCaseSeverity" => {
                        builder = builder.set_notify_on_case_severity(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| {
                                s.to_unescaped().map(|u| {
                                    crate::model::NotificationSeverityLevel::from(u.as_ref())
                                })
                            })
                            .transpose()?,
                        );
                    }
                    "notifyOnCreateOrReopenCase" => {
                        builder = builder.set_notify_on_create_or_reopen_case(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(
                                tokens.next(),
                            )?,
                        );
                    }
                    "notifyOnResolveCase" => {
                        builder = builder.set_notify_on_resolve_case(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(
                                tokens.next(),
                            )?,
                        );
                    }
                    "teamId" => {
                        builder = builder.set_team_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(aws_smithy_json::deserialize::Error::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_com_amazonaws_supportapp_slack_channel_configuration_list<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<std::vec::Vec<crate::model::SlackChannelConfiguration>>,
    aws_smithy_json::deserialize::Error,
>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value =
                            crate::json_deser::deser_structure_crate_model_slack_channel_configuration(tokens)?
                        ;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(aws_smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_com_amazonaws_supportapp_slack_workspace_configuration_list<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<std::vec::Vec<crate::model::SlackWorkspaceConfiguration>>,
    aws_smithy_json::deserialize::Error,
>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value =
                            crate::json_deser::deser_structure_crate_model_slack_workspace_configuration(tokens)?
                        ;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(aws_smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}

pub fn deser_structure_crate_model_slack_channel_configuration<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::SlackChannelConfiguration>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::SlackChannelConfiguration::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "teamId" => {
                                builder = builder.set_team_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "channelId" => {
                                builder = builder.set_channel_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "channelName" => {
                                builder = builder.set_channel_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "notifyOnCreateOrReopenCase" => {
                                builder = builder.set_notify_on_create_or_reopen_case(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "notifyOnAddCorrespondenceToCase" => {
                                builder = builder.set_notify_on_add_correspondence_to_case(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "notifyOnResolveCase" => {
                                builder = builder.set_notify_on_resolve_case(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "notifyOnCaseSeverity" => {
                                builder = builder.set_notify_on_case_severity(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::model::NotificationSeverityLevel::from(
                                                u.as_ref(),
                                            )
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "channelRoleArn" => {
                                builder = builder.set_channel_role_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(aws_smithy_json::deserialize::Error::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(aws_smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_crate_model_slack_workspace_configuration<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::SlackWorkspaceConfiguration>, aws_smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::SlackWorkspaceConfiguration::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "teamId" => {
                                builder = builder.set_team_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(aws_smithy_json::deserialize::Error::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(aws_smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}