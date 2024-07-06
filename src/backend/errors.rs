use thiserror::Error;


#[derive(Debug, Error)]
pub enum BackendError {
    #[error(transparent)]
    Init(BackendInitError)
}

#[derive(Debug, Error)]
pub enum BackendInitError {
    #[error("No windowing backend could be created! Unable to continue.")]
    NoBackend
}