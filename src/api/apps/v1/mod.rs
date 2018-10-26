mod statefulset;
mod deployment;

pub use self::deployment::*;
pub use self::statefulset::*;

use super::API_GROUP;
pub const API_VERSION: &str = "v1";
