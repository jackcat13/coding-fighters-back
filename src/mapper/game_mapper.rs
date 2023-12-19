use crate::dto::game_dto::GameDto;
use crate::model::game::Game;

pub fn to_entity(game_dto: GameDto) -> Game {
    Game {
        id: None,
        topics: game_dto.topics,
        question_number: game_dto.question_number,
        is_private: game_dto.is_private,
    }
}

pub fn to_dto(game: Game) -> GameDto {
    GameDto {
        id: Some(game.id.expect("Failed to get game id").to_string()),
        topics: game.topics,
        question_number: game.question_number,
        is_private: game.is_private,
    }
}
