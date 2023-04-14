use serenity::{
    model::prelude::{
        application_command::{
            ApplicationCommandInteraction, ApplicationCommandInteractionData,
            ApplicationCommandInteractionDataOptionValue,
        },
        InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
    },
    prelude::Context,
    Error,
};

pub async fn error_response<D>(
    ctx: &Context,
    cmd: ApplicationCommandInteraction,
    message: D,
) -> Result<(), Error>
where
    D: ToString,
{
    cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
                msg.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                msg.content(message)
            })
    })
    .await
}

pub fn get_param_as_str<'a>(
    data: &'a ApplicationCommandInteractionData,
    index: usize,
) -> Option<&'a str> {
    data.options
        .get(index)
        .and_then(|x| x.value.as_ref())
        .and_then(|x| x.as_str())
}

pub fn get_param_as_bool(data: &ApplicationCommandInteractionData, index: usize) -> Option<bool> {
    data.options.get(index).map(|usr| match &usr.resolved {
        Some(ApplicationCommandInteractionDataOptionValue::Boolean(val)) => val.to_owned(),
        _ => false,
    })
}
