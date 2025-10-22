use zed_extension_api::{self as zed};

struct MechCommitExtension;

impl zed::Extension for MechCommitExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }
}

zed::register_extension!(MechCommitExtension);
