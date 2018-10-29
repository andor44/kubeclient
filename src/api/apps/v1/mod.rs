mod statefulset;
mod deployment;
mod daemonset;
mod replicaset;

pub use self::deployment::*;
pub use self::statefulset::*;
pub use self::daemonset::*;
pub use self::replicaset::*;

use super::API_GROUP;
pub const API_VERSION: &str = "v1";
