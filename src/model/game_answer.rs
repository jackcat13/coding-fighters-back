use rocket::serde::{Deserialize, Serialize};

use crate::model::game_progress::Question;

///GameAnswer entity to be stored in the database
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameAnswer {
    pub game_id: String,
    pub user: String,
    pub answer: i8,
    pub question_index: i8,
    pub correct_answer: i8,
    pub(crate) question: Question,
}
