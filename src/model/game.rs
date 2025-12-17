use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

///Game entity to be stored in the database
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub topics: Vec<String>,
    pub question_number: i8,
    pub is_private: bool,
    pub is_started: bool,
    pub creator: Option<String>,
    pub users: Vec<String>,
}
