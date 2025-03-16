#[cfg(not(target_arch = "wasm32"))]
use async_std_resolver::ResolveError;

#[derive(Debug)]
pub enum RbError {
    Reqwest(reqwest::Error),
    #[cfg(not(target_arch = "wasm32"))]
    Resolve(ResolveError),
}

impl std::error::Error for RbError {}

impl std::fmt::Display for RbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RbError::Reqwest(err) => write!(f, "Reqwest: {}", err),
            #[cfg(not(target_arch = "wasm32"))]
            RbError::Resolve(err) => write!(f, "Resolve: {}", err),
        }
    }
}

impl From<reqwest::Error> for RbError {
    fn from(value: reqwest::Error) -> Self {
        RbError::Reqwest(value)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<ResolveError> for RbError {
    fn from(value: ResolveError) -> Self {
        RbError::Resolve(value)
    }
}
