mod bot;
mod commands;
mod helpers;
mod questions;

use anyhow::Result;
use dotenv::dotenv;
use questions::{load_questions, QuestionsContainer};
use serenity::prelude::*;
use std::{collections::HashMap, env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let token =
        env::var("ECLIPSE_BOT_TOKEN").expect("Expected ECLIPSE_BOT_TOKEN in the environment");

    let application_id: u64 = env::var("ECLIPSE_BOT_APPLICATION_ID")
        .expect("Expected ECLIPSE_BOT_APPLICATION_ID in the environment")
        .parse()
        .expect("ECLIPSE_BOT_APPLICATION_ID is not a valid id");

    let mut client = Client::builder(token)
        .event_handler(bot::Bot)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    let mut faq = HashMap::new();
    load_questions(&mut faq).await?;
    {
        let mut data = client.data.write().await;
        data.insert::<QuestionsContainer>(Arc::new(Mutex::new(faq)));
    }

    client.start().await?;
    Ok(())
}
