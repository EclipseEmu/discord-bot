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

pub struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let result = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command
                        .name("faq-reload")
                        .description("Reloads the FAQ list")
                        .default_permission(false)
                })
                .create_application_command(|command| {
                    command
                        .name("faq-list")
                        .description("Get a list of all of the question IDs.")
                })
                .create_application_command(|command| {
                    command
                        .name("faq")
                        .description("Answer a frequently asked question")
                        .create_option(|option| {
                            option
                                .name("id")
                                .description("The question's ID to use. See /faq-list for all IDs.")
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

        if let Err(why) = result {
            eprintln!("error registering global commands: {why:?}");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let response = match interaction {
            Interaction::ApplicationCommand(cmd) => match cmd.data.name.as_str() {
                "ping" => commands::ping::handle(ctx, cmd).await,
                "faq-reload" => commands::reload::handle(ctx, cmd).await,
                "faq" => commands::faq::handle(ctx, cmd).await,
                "faq-list" => commands::faq_list::handle(ctx, cmd).await,
                _ => unreachable!(),
            },
            Interaction::Autocomplete(ac) => match ac.data.name.as_str() {
                "faq" => commands::faq::autocomplete(ctx, ac).await,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        if let Err(why) = response {
            eprintln!("interaction error: {why:?}");
        }
    }
}
