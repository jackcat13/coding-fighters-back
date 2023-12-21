
pub struct GameServiceError {
    pub message: String,
    pub kind: GameServiceErrorKind,
}

pub enum GameServiceErrorKind {
    NotFound,
    Internal,
}
