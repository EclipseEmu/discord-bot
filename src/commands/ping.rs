use serenity::{
    model::interactions::{
        application_command::ApplicationCommandInteraction,
        InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
    },
    prelude::Context,
    Error,
};

pub async fn handle(ctx: Context, cmd: ApplicationCommandInteraction) -> Result<(), Error> {
    cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
                msg.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                msg.content("pong")
            })
    })
    .await
}
