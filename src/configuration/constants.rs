use crate::prelude::*;


/// Storage format of configuration files.
pub const CONFIGURATION_STORAGE_FORMAT: StorageFormat = {
    #[cfg(feature = "native")]
    {
        StorageFormat::Toml
    }
    #[cfg(feature = "wasm")]
    {
        StorageFormat::Json
    }
};
