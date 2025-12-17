use rocket::serde::{Deserialize, Serialize};

///GameDto is used to interact with the game frontend in the [GameResource](crate::resource::game_resource::GameResource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub topics: Vec<String>,
    pub question_number: i8,
    pub is_private: bool,
    pub is_started: bool,
    pub creator: Option<String>,
    #[serde(skip)]
    pub users: Vec<String>,
}
