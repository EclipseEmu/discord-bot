use serenity::{
    model::interactions::{
        application_command::ApplicationCommandInteraction,
        InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
    },
    prelude::Context,
    Error,
};

use crate::{
    helpers,
    questions::{get_questions, load_questions, QuestionsContainer},
};

pub async fn handle(ctx: Context, cmd: ApplicationCommandInteraction) -> Result<(), Error> {
    let guard = ctx.data.read().await;
    let Ok(mut map) = get_questions!(guard) else {
    	return helpers::error_response(&ctx, cmd, "An error occurred while trying to load the questions, try again later.").await;
    };

    match load_questions(&mut map).await {
        Ok(_) => {
            cmd.create_interaction_response(&ctx.http, |resp| {
                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                        msg.content("Successfully reloaded the questions!")
                    })
            })
            .await
        }
        Err(_) => {
            cmd.create_interaction_response(&ctx.http, |resp| {
                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                        msg.content("Failed to reload the questions...")
                    })
            })
            .await
        }
    }
}
