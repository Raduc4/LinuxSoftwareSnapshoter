use thiserror::Error;

#[derive(Debug, Eq, Error, PartialEq)]
pub enum DetectorError {
    #[error("Failed to detect the OS distribution")]
    OsDetectionError,

    #[error("Failed to detect the OS distribution name")]
    OsNameDetectionError,

    #[error("Failed to detect the OS version")]
    OsVersionDetectionError,

    #[error("Failed to detect the architecture")]
    ArchDetectionError,

    #[error("Failed to detect the package manager: {0}")]
    PackageManagerDetectionError(String),
}
