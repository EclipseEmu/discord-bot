use serenity::{
    model::interactions::{
        application_command::ApplicationCommandInteraction,
        InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
    },
    prelude::Context,
    Error,
};

use crate::{get_questions, helpers, questions::QuestionsContainer};

pub async fn handle(ctx: Context, cmd: ApplicationCommandInteraction) -> Result<(), Error> {
    let guard = ctx.data.read().await;
    let Ok(map) = get_questions!(guard) else {
    	return helpers::error_response(&ctx, cmd, "An error occurred while trying to load the questions, try again later.").await;
    };
    let identifiers = map
        .keys()
        .map(String::to_owned)
        .collect::<Vec<String>>()
        .join("\n");
    cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
                msg.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                msg.create_embed(|embed| {
                    embed.title("Question IDs");
                    embed.description(identifiers)
                })
            })
    })
    .await
}
