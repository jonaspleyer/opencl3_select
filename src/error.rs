use thiserror::Error;

/// Wrapper for [std::result::Result] containing custom [ClSelectError].
pub type Result<T> = std::result::Result<T, ClSelectError>;

/// Contains all possible error variants. Generated with [thiserror]
#[derive(Error, Debug)]
pub enum ClSelectError {
    /// unable to get opencl info
    #[error("unable to get opencl info")]
    OpenCL(#[from] opencl3::error_codes::ClError),

    /// failed to display
    #[error("failed to display")]
    #[cfg(feature = "display")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "display")))]
    Display(#[from] std::io::Error),

    /// error during (de)serialization
    #[cfg(feature = "serde")]
    #[error("error during (de)serialization")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
    Deserialize(#[from] serde::de::value::Error),
}
