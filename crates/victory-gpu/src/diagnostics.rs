//! Host-independent parsing of the diagnostics environment switches.
//!
//! The GPU host is Windows-only, but the meaning of a switch is not. Keeping
//! the predicate here lets its behaviour be tested on every platform.

use std::ffi::OsStr;

/// Whether `VICTORY_D3D12_WARP` asks for the software adapter. An explicit
/// `0` means unset, so a launcher can leave the variable in place and flip
/// its value without deleting it (issue #610).
pub(crate) fn warp_requested(value: Option<&OsStr>) -> bool {
    value.is_some_and(|value| value != "0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_like_an_unset_switch() {
        assert!(!warp_requested(None));
        assert!(!warp_requested(Some(OsStr::new("0"))));
        assert!(warp_requested(Some(OsStr::new("1"))));
        assert!(warp_requested(Some(OsStr::new(""))));
        assert!(warp_requested(Some(OsStr::new("yes"))));
    }
}
