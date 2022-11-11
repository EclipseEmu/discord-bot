use crate::commands;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            application_command::{ApplicationCommand, ApplicationCommandOptionType},
            Interaction,
        },
    },
    prelude::*,
};

pub use super::faq_list::*;

pub struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Set the commands on the guild
        let _ = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command
                        .name("faq-list")
                        .description("Get a list of all FAQ IDs.")
                })
                .create_application_command(|command| {
                    command
                        .name("faq")
                        .description("Answer a frequently asked question")
                        .create_option(|option| {
                            option
                                .name("id")
                                .description("The FAQ ID to use. See /faq-list for all IDs.")
                                .set_autocomplete(true)
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("show")
                                .description("Show the answer in the current channel?")
                                .kind(ApplicationCommandOptionType::Boolean)
                        })
                })
        })
        .await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // Handle Interactions
        let response = match interaction {
            Interaction::ApplicationCommand(command) => match command.data.name.as_str() {
                "ping" => commands::ping(ctx, command).await,
                "faq" => commands::faq(ctx, command).await,
                "faq-list" => commands::faq_list(ctx, command).await,
                _ => unreachable!(),
            },
            Interaction::Autocomplete(autocomplete) => match autocomplete.data.name.as_str() {
                "faq" => commands::faq_handle_autocomplete(ctx, autocomplete).await,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        if let Err(why) = response {
            eprintln!("interaction error: {why:?}");
        }
    }
}
