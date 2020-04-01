use crate::schema::{embeds, messages, reminders, channels, users};

#[derive(Identifiable, Queryable, Serialize)]
#[table_name = "embeds"]
pub struct Embed {
    #[serde(skip)]
    pub id: u32,

    pub title: String,
    pub description: String,
    pub color: u32,
}

#[derive(Identifiable, Queryable)]
#[table_name = "messages"]
pub struct Message {
    pub id: u32,

    pub content: String,
    pub embed_id: Option<u32>,
}

#[derive(Identifiable, Queryable)]
#[table_name = "reminders"]
pub struct Reminder {
    pub id: u32,
    pub uid: String,

    pub message_id: u32,

    pub channel_id: Option<u32>,
    pub user_id: Option<u32>,

    pub time: u32,
    pub interval: Option<u32>,
    pub enabled: bool,

    pub avatar: String,
    pub username: String,

    pub method: Option<String>,
}

#[derive(Identifiable, Queryable)]
#[table_name = "channels"]
pub struct Channel {
    pub id: u32,
    pub channel: u64,

    pub nudge: i16,
    pub blacklisted: bool,

    pub name: Option<String>,

    pub webhook_id: Option<u64>,
    pub webhook_token: Option<String>,

    pub guild_id: u32,
}

#[derive(Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: u32,
    pub user: u64,

    pub name: String,

    pub language: String,
    pub timezone: Option<String>,
    pub allowed_dm: bool,

    pub patreon: bool,

    pub dm_channel: u64,
}
