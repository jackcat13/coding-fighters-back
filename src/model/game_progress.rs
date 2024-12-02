use rocket::serde::{Deserialize, Serialize};

///GameProgress entity to be stored in the database
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameProgress {
    #[serde(rename = "_id")]
    pub id: String,
    pub current_question: i8,
    pub question_number: i8,
    pub question_content: Question,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Question {
    pub question_text: String,
    pub answer_1: String,
    pub answer_2: String,
    pub answer_3: String,
    pub answer_4: String,
    pub good_answer_number: i8,
    pub topic: String,
}
