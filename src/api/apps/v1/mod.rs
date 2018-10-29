mod statefulset;
mod deployment;
mod daemonset;

pub use self::deployment::*;
pub use self::statefulset::*;
pub use self::daemonset::*;

use super::API_GROUP;
pub const API_VERSION: &str = "v1";
