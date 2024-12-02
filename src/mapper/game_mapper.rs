use crate::dto::answer::GameAnswerDto;
use crate::dto::game_dto::GameDto;
use crate::dto::game_progress_dto::GameProgressDto;
use crate::model::game::Game;
use crate::model::game_answer::GameAnswer;
use crate::model::game_progress::{GameProgress, Question};
use mongodb::bson::oid::ObjectId;
use std::str::FromStr;

///this mapper is used to map between the [GameDto](crate::dto::game_dto::GameDto) and [Game](crate::model::game::Game) models
pub fn to_entity(game_dto: GameDto) -> Game {
    let id = game_dto
        .id
        .map(|id| ObjectId::from_str(id.as_str()).expect("Failed to get object id from id string"));
    Game {
        id,
        topics: game_dto.topics,
        question_number: game_dto.question_number,
        is_private: game_dto.is_private,
        is_started: game_dto.is_started,
        creator: game_dto.creator,
    }
}

///this mapper is used to map between the [Game](crate::model::game::Game) and [GameDto](crate::dto::game_dto::GameDto) models
pub fn to_dto(game: Game) -> GameDto {
    GameDto {
        id: Some(game.id.expect("Failed to get game id").to_string()),
        topics: game.topics,
        question_number: game.question_number,
        is_private: game.is_private,
        is_started: game.is_started,
        creator: game.creator,
    }
}

pub fn progress_to_entity(game_progress_dto: GameProgressDto) -> GameProgress {
    let question = game_progress_dto.question_content;
    GameProgress {
        id: game_progress_dto.game_id,
        current_question: game_progress_dto.current_question,
        question_number: game_progress_dto.question_number,
        question_content: Question {
            question_text: question.question_text,
            answer_1: question.answer_1,
            answer_2: question.answer_2,
            answer_3: question.answer_3,
            answer_4: question.answer_4,
            good_answer_number: question.good_answer_number,
            topic: question.topic,
        },
    }
}

pub fn answer_to_entity(game_answer_dto: GameAnswerDto) -> GameAnswer {
    GameAnswer {
        game_id: game_answer_dto.game_id,
        answer: game_answer_dto.answer,
        user: game_answer_dto.user,
        question_index: game_answer_dto.question_index,
    }
}
