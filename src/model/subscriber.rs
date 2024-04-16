use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
use crate::model::notification::Notification; 

#[derive(Debug, Serialize, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subsriber {
    pub url: String,
    pub name: String,
}