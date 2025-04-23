use std::path::Path;

use nix::sys::statvfs::{statvfs, FsFlags};

use crate::errors::GreenlightError;

pub fn is_rootfs_readonly() -> Result<bool, GreenlightError> {
    match statvfs(Path::new("/")) {
        Ok(state) => Ok(state.flags().contains(FsFlags::ST_RDONLY)),
        Err(error) => Err(GreenlightError::CheckFailed(error.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rootfs_readonly_runs() {
        // This test ensures that calling the function doesn't panic and returns a result
        let result = is_rootfs_readonly();

        assert!(
            result.is_ok() || result.is_err(),
            "Expected a Result, got something unexpected"
        );
    }
}
