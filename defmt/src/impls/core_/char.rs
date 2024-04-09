use core::char;

use super::*;

impl Format for char::TryFromCharError {
    fn format(&self, fmt: Formatter) {
        crate::write!(fmt, "TryFromCharError(())");
    }
}
