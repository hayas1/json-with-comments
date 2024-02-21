pub mod de;
pub mod error;

pub use de::from::{from_file, from_path, from_read};
pub use error::{JsonWithCommentError as Error, Result};
