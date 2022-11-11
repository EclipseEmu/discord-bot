use serenity::{
    model::interactions::{
        application_command::{
            ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
        },
        autocomplete::AutocompleteInteraction,
        InteractionApplicationCommandCallbackDataFlags,
    },
    prelude::Context,
    Error,
};

use crate::bot::{FaqEntry, FaqListStore};

pub async fn faq(ctx: Context, cmd: ApplicationCommandInteraction) -> Result<(), Error> {
    let id = cmd
        .data
        .options
        .get(0)
        .and_then(|x| x.value.as_ref())
        .and_then(|x| x.as_str());

    let show = cmd
        .data
        .options
        .get(1)
        .map(|usr| match &usr.resolved {
            Some(ApplicationCommandInteractionDataOptionValue::Boolean(val)) => val,
            _ => &false,
        })
        .unwrap_or(&false);

    // read the faq entry
    let data_lock = ctx.data.read().await;
    let faq_entry = data_lock.get::<FaqListStore>().and_then(|lock| {
        if let Some(id) = id {
            lock.get(id)
        } else {
            None
        }
    });

    cmd.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|msg| {
            if !show || faq_entry.is_none() {
                msg.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
            }
            msg.create_embed(|embed| {
                if let Some(faq_entry) = faq_entry {
                    embed
                        .title(&faq_entry.title)
                        .description(&faq_entry.description);
                    if let Some(url) = &faq_entry.url {
                        embed.url(url);
                    }
                    embed
                } else {
                    embed
						.title("Unknown FAQ entry")
						.description("The FAQ ID you entered doesn't seem to exist. See /faq-list for a list of entries!")
                }
            })
        })
    })
    .await
}

pub async fn faq_handle_autocomplete(
    ctx: Context,
    req: AutocompleteInteraction,
) -> Result<(), Error> {
    let data_lock = ctx.data.read().await;
    let map = data_lock.get::<FaqListStore>();
    req.create_autocomplete_response(&ctx.http, |resp| {
        let data = req
            .data
            .options
            .get(0)
            .and_then(|x| x.value.as_ref())
            .and_then(|x| x.as_str());
        if let Some(query) = data {
            if let Some(map) = map {
                let lower_query = query.to_lowercase();
                let filtered_iter = map
                    .iter()
                    .filter(|(key, entry)| faq_filter_entries(&lower_query, key, entry));
                for (key, entry) in filtered_iter {
                    resp.add_string_choice(&entry.title, key);
                }
            }
        }
        resp
    })
    .await
}

fn faq_filter_entries(query: &String, key: &&String, entry: &&FaqEntry) -> bool {
    key.to_lowercase().contains(query) || entry.title.to_lowercase().contains(query)
}
