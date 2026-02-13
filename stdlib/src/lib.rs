//! Zenith Standard Library
//! Comprehensive standard library for Zenith programming language

// Core modules
pub mod core;

// Re-export commonly used types and functions
pub use core::primitives::{
    BinaryOperator, Expression, Primitives, Statement, UnaryOperator, Value,
};

pub use core::collections::{
    CollectionUtils, ZenithHashMap, ZenithHashSet, ZenithLinkedList, ZenithPriorityQueue,
    ZenithQueue, ZenithStack, ZenithVector,
};

pub use core::io;
pub use core::math::{Math, Statistics, Trigonometry};
pub use core::net;
pub use core::os;
pub use core::ui;

// Version information
pub const VERSION: &str = "0.1.0";
pub const VERSION_MAJOR: u32 = 0;
pub const VERSION_MINOR: u32 = 1;
pub const VERSION_PATCH: u32 = 0;

// Library metadata
pub const NAME: &str = "Zenith Standard Library";
pub const DESCRIPTION: &str = "Comprehensive standard library for Zenith programming language";
pub const AUTHORS: &str = "Zenith Language Team";
pub const HOMEPAGE: &str = "https://zenith-lang.org";
pub const REPOSITORY: &str = "https://github.com/zenith-lang/zenith";

/// Get library version as string
pub fn version_string() -> String {
    format!("{}.{}", VERSION_MAJOR, VERSION_MINOR)
}

/// Get full version including patch
pub fn full_version_string() -> String {
    format!("{}.{}", version_string(), VERSION_PATCH)
}

/// Check if version is compatible
pub fn is_compatible(major: u32, minor: u32) -> bool {
    major == VERSION_MAJOR && minor <= VERSION_MINOR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        assert_eq!(VERSION, "0.1.0");
        assert_eq!(VERSION_MAJOR, 0);
        assert_eq!(VERSION_MINOR, 1);
        assert_eq!(VERSION_PATCH, 0);

        assert_eq!(version_string(), "0.1");
        assert_eq!(full_version_string(), "0.1.0");

        assert!(is_compatible(0, 1));
        assert!(is_compatible(0, 0));
        assert!(!is_compatible(1, 0));
        assert!(!is_compatible(0, 2));
    }
}
