use serde::Deserialize;
use serenity::{prelude::TypeMapKey, Client};
use std::{collections::HashMap, fs::File, io::Read, sync::Arc};
// use tokio::sync::RwLock;

pub struct FaqListStore;

#[derive(Debug, Deserialize)]
pub struct FaqEntry {
    pub title: String,
    pub description: String,
    pub url: Option<String>,
}

impl TypeMapKey for FaqListStore {
    type Value = Arc<HashMap<String, FaqEntry>>;
}

pub async fn load_faq_list(client: &Client, file: &mut File) {
    let mut data = client.data.write().await;
    let mut string = Vec::new();
    file.read_to_end(&mut string)
        .expect("expected to be able to load the faq file");
    let faq_entries: HashMap<String, FaqEntry> = toml::from_slice(string.as_slice())
        .expect("expected to be able to parse the faq file toml");
    data.insert::<FaqListStore>(Arc::new(faq_entries));
}
