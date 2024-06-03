use crate::dto::game_dto::GameDto;
use crate::model::game::Game;

///this mapper is used to map between the [GameDto](crate::dto::game_dto::GameDto) and [Game](crate::model::game::Game) models
pub fn to_entity(game_dto: GameDto) -> Game {
    Game {
        id: None,
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
