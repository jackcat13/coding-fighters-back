/// Error type for GameService.
/// It is thrown anytime an error occurs in the service.
pub struct GameServiceError {
    pub message: String,
    pub kind: GameServiceErrorKind,
}

/// Provides possible error kinds happening in the service.
pub enum GameServiceErrorKind {
    NotFound,
    Internal,
}
