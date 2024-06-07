use rocket::serde::{Deserialize, Serialize};
use std::string::ToString;

///GameProgressDto is used to interact with the game frontend in the [GameResource](crate::resource::game_resource::GameResource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameProgressDto {
    pub game_id: String,
    pub current_question: i8,
    pub question_number: i8,
    pub question_content: QuestionDto,
}

///QuestionDto is used to interact with the game frontend in the [GameResource](crate::resource::game_resource::GameResource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuestionDto {
    pub question_text: String,
    pub answer_1: String,
    pub answer_2: String,
    pub answer_3: String,
    pub answer_4: String,
    pub good_answer_number: i8,
    pub topic: String,
}

pub fn questions() -> Vec<QuestionDto> {
    vec![
        QuestionDto {
            question_text: "What type is not a primitive data type ?".to_string(),
            answer_1: "long".to_string(),
            answer_2: "double".to_string(),
            answer_3: "String".to_string(),
            answer_4: "char".to_string(),
            good_answer_number: 3,
            topic: "Java".to_string(),
        },
        QuestionDto {
            question_text: "What statement is true ?".to_string(),
            answer_1: "A class may implement multiple interfaces and may extend one class maximum"
                .to_string(),
            answer_2:
                "A class may implement multiple classes but is allowed to extend only one interface"
                    .to_string(),
            answer_3:
                "A class may implement multiple classes and may implement multiple interfaces"
                    .to_string(),
            answer_4: "A class must implement one interface and must extend one class".to_string(),
            good_answer_number: 1,
            topic: "Java".to_string(),
        },
    ]
}
