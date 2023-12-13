use crate::dto::game_dto::GameDto;
use crate::model::game::Game;

pub async fn to_entity(game_dto: GameDto) -> Game {
    Game {
        id: None,
        topics: game_dto.topics,
        question_number: game_dto.question_number,
    }
}

pub async fn to_dto(game: Game) -> GameDto {
    GameDto {
        id: game.id,
        topics: game.topics,
        question_number: game.question_number,
    }
}
