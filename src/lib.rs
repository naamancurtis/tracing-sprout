pub(crate) mod constants;
mod error;
mod formatting;
mod storage;
pub(crate) mod util;

pub use error::SproutError;
pub use formatting::TrunkLayer;

pub type Result<T> = std::result::Result<T, SproutError>;
