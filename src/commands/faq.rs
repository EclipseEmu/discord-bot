use serenity::{
    model::interactions::{
        application_command::ApplicationCommandInteraction, autocomplete::AutocompleteInteraction,
        InteractionApplicationCommandCallbackDataFlags,
    },
    prelude::Context,
    Error,
};

use crate::{
    get_questions, helpers,
    questions::{QuestionEntry, QuestionsContainer},
};

pub async fn handle(ctx: Context, cmd: ApplicationCommandInteraction) -> Result<(), Error> {
    let Some(id) = helpers::get_param_as_str(&cmd.data, 0) else {
	    return helpers::error_response(&ctx, cmd, "No question ID was provided, you shouldn't be able to do that...").await;
    };

    let show = helpers::get_param_as_bool(&cmd.data, 1).unwrap_or(false);

    let guard = ctx.data.read().await;
    let Ok(map) = get_questions!(guard) else {
    	return helpers::error_response(&ctx, cmd, "An error occurred while trying to load the questions, try again later.").await;
    };

    let faq_entry = map.get(id);

    cmd.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|msg| {
            if !show || faq_entry.is_none() {
                msg.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
            }
            msg.create_embed(|embed| match faq_entry {
                Some(entry) => embed.title(&entry.q).description(&entry.a),
                _ => embed
                    .title("Unknown Question")
                    .description("The question you requested doesn't seem to exist. See /faq-list for a list of question IDs!")
            })
        })
    })
    .await
}

pub async fn autocomplete(ctx: Context, req: AutocompleteInteraction) -> Result<(), Error> {
    let guard = ctx.data.read().await;
    let Ok(map) = get_questions!(guard) else {
    	return Ok(());
    };
    req.create_autocomplete_response(&ctx.http, |resp| {
        let Some(query) = helpers::get_param_as_str(&req.data, 0) else {
            return resp;
        };

        let query = query.to_lowercase();
        for entry in map
            .values()
            .filter(|entry| faq_filter_entries(&query, entry))
        {
            resp.add_string_choice(&entry.q, &entry.id);
        }
        resp
    })
    .await
}

fn faq_filter_entries(query: &String, entry: &&QuestionEntry) -> bool {
    entry.id.to_lowercase().contains(query) || entry.q.to_lowercase().contains(query)
}
