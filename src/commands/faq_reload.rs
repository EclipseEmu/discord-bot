use serenity::{
    model::interactions::application_command::ApplicationCommandInteraction, prelude::Context,
    Error,
};

use crate::{
    helpers,
    questions::{get_questions, load_questions, QuestionsContainer, FAILED_LOAD_RESPONSE},
};

pub async fn handle(ctx: Context, cmd: ApplicationCommandInteraction) -> Result<(), Error> {
    let guard = ctx.data.read().await;
    let Ok(mut map) = get_questions!(guard) else {
    	return helpers::simple_response(&ctx, cmd, FAILED_LOAD_RESPONSE).await;
    };

    let Ok(_) = load_questions(&mut map).await else {
	    return helpers::simple_response(&ctx, cmd, "Failed to reload the questions").await;
    };
    helpers::simple_response(&ctx, cmd, "Successfully reloaded the questions").await
}
