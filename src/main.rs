mod bot;
mod commands;
mod faq_list;

use dotenv::dotenv;
use faq_list::load_faq_list;
use serenity::prelude::*;
use std::{env, fs::File};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let faq_path =
        env::var("ECLIPSE_FAQ_FILE").expect("Expected ECLIPSE_FAQ_FILE in the environment");
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

    let mut faq_file = File::open(faq_path).expect("Expected to be able to open the file");
    load_faq_list(&client, &mut faq_file).await;

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
