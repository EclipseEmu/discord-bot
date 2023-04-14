use serenity::{
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context, Error,
};

use crate::helpers;

pub async fn handle(ctx: Context, cmd: ApplicationCommandInteraction) -> Result<(), Error> {
    helpers::simple_response(&ctx, cmd, "pong").await
}
