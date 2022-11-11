use serenity::{
    model::interactions::{
        application_command::ApplicationCommandInteraction,
        InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
    },
    prelude::Context,
    Error,
};

use crate::bot::FaqListStore;

pub async fn faq_list(ctx: Context, cmd: ApplicationCommandInteraction) -> Result<(), Error> {
    let data_lock = ctx.data.read().await;
    let lock = data_lock.get::<FaqListStore>();
    cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
                msg.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                msg.create_embed(|embed| {
                    embed.title("FAQ IDs");
                    if let Some(map) = lock {
                        embed.description(format!("{:?}", map.keys().collect::<Vec<&String>>()))
                    } else {
                        embed.description("An error occurred while loading the FAQ IDs...")
                    }
                })
            })
    })
    .await
}
