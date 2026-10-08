use thiserror::Error;



#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Can not find config in directory: ${0}")]
    WrongPathError(String),
    #[error("Can not parse config due to: ${0}")]
    ParsingError(String),
    #[error("Can not write config due to: ${0}")]
    WritingError(String),
}

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("Game loop error occurred: {0}")]
    GameLoopError(String),
    #[error("Config reading error occurred: {0}")]
    ConfigReadingError(#[from] ConfigError),
    #[error("Error occurred while trying read the state")]
    StateLockError,
    #[error("Level prefab rows have inconsistent widths")]
    InconsistentLevelWidth,
}
