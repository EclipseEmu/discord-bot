use serenity::{
    model::application::interaction::{
        application_command::{ApplicationCommandInteraction, CommandData, CommandDataOptionValue},
        InteractionResponseType, MessageFlags,
    },
    prelude::Context,
    Error,
};

pub async fn simple_response<D>(
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
                msg.flags(MessageFlags::EPHEMERAL);
                msg.content(message)
            })
    })
    .await
}

pub fn get_param_as_str<'a>(data: &'a CommandData, index: usize) -> Option<&'a str> {
    data.options
        .get(index)
        .and_then(|x| x.value.as_ref())
        .and_then(|x| x.as_str())
}

pub fn get_param_as_bool(data: &CommandData, index: usize) -> Option<bool> {
    data.options.get(index).map(|usr| match &usr.resolved {
        Some(CommandDataOptionValue::Boolean(val)) => val.to_owned(),
        _ => false,
    })
}
