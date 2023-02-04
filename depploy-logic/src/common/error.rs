use std::fmt;
use std::error;

//type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub enum PTGenError { Exists, ServerError, ConfigNotFound, PresistingError, FailedBuilding, FailedPushing }

impl fmt::Display for PTGenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         match self {
            PTGenError::Exists => write!(f, "Your project already has a configured prototype deployed."),
            PTGenError::ServerError => write!(f, "An error occurred on the server side"),
            PTGenError::PresistingError => write!(f, "Could not presist prototype details, check your file permisson."),
            PTGenError::ConfigNotFound => write!(f, "Could not finde the conf.toml in your project."),
            PTGenError::FailedBuilding => write!(f, "Could not build docker container."),
            PTGenError::FailedPushing => write!(f, "Failed pushing container"),
        }
    }
}
impl error::Error for PTGenError {}