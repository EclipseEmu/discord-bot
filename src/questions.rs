use anyhow::Result;
use serde::Deserialize;
use serenity::{prelude::Mutex, prelude::TypeMapKey};
use std::{collections::HashMap, sync::Arc};

pub struct QuestionsContainer;

pub const FAILED_LOAD_RESPONSE: &'static str =
    "An error occurred while trying to load the questions, try again later.";

#[derive(Debug, Deserialize)]
pub struct QuestionEntry {
    pub id: String,
    pub q: String,
    pub a: String,
}

impl TypeMapKey for QuestionsContainer {
    type Value = Arc<Mutex<HashMap<String, QuestionEntry>>>;
}

pub async fn load_questions(map: &mut HashMap<String, QuestionEntry>) -> Result<()> {
    let faq_url =
        std::env::var("ECLIPSE_FAQ_URL").unwrap_or("https://eclipseemu.me/faq.json".to_owned());
    let entries = reqwest::get(faq_url)
        .await?
        .json::<Vec<QuestionEntry>>()
        .await?;
    map.clear();
    for entry in entries {
        map.insert(entry.id.clone(), entry);
    }
    Ok(())
}

#[macro_export]
macro_rules! get_questions {
    ($guard:expr) => {
        match $guard.get::<QuestionsContainer>() {
            Some(lock) => match lock.try_lock() {
                Ok(map) => Ok(map),
                Err(_) => Err(anyhow::anyhow!("failed to aquire a lock on the questions")),
            },
            None => Err(anyhow::anyhow!("failed to get the questions container")),
        }
    };
}

pub use get_questions;
