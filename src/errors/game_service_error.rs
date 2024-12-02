/// Error type for GameService.
/// It is thrown anytime an error occurs in the service.
#[derive(Debug)]
pub struct GameServiceError {
    pub message: String,
    pub kind: GameServiceErrorKind,
}

/// Provides possible error kinds happening in the service.
#[derive(Debug)]
pub enum GameServiceErrorKind {
    NotFound,
    Internal,
}
