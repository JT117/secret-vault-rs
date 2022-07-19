use rsb_derive::Builder;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SecretVaultError {
    SystemError(SecretVaultSystemError),
    DataNotFoundError(SecretVaultDataNotFoundError),
    InvalidParametersError(SecretVaultInvalidParametersError),
    NetworkError(SecretVaultNetworkError),
    EncryptionError(SecretVaultEncryptionError),
    MemoryError(SecretVaultMemoryError),
}

impl Display for SecretVaultError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            SecretVaultError::SystemError(ref err) => err.fmt(f),
            SecretVaultError::DataNotFoundError(ref err) => err.fmt(f),
            SecretVaultError::InvalidParametersError(ref err) => err.fmt(f),
            SecretVaultError::NetworkError(ref err) => err.fmt(f),
            SecretVaultError::EncryptionError(ref err) => err.fmt(f),
            SecretVaultError::MemoryError(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for SecretVaultError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SecretVaultError::SystemError(ref err) => Some(err),
            SecretVaultError::DataNotFoundError(ref err) => Some(err),
            SecretVaultError::InvalidParametersError(ref err) => Some(err),
            SecretVaultError::NetworkError(ref err) => Some(err),
            SecretVaultError::EncryptionError(ref err) => Some(err),
            SecretVaultError::MemoryError(ref err) => Some(err),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SecretVaultErrorPublicGenericDetails {
    pub code: String,
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SecretVaultSystemError {
    pub public: SecretVaultErrorPublicGenericDetails,
    pub message: String,
}

impl Display for SecretVaultSystemError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "SecretVault system/internal error: {:?} / {}",
            self.public, self.message
        )
    }
}

impl std::error::Error for SecretVaultSystemError {}

#[derive(Debug, Clone, Builder)]
pub struct SecretVaultDatabaseError {
    pub public: SecretVaultErrorPublicGenericDetails,
    pub details: String,
    pub retry_possible: bool,
}

impl Display for SecretVaultDatabaseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Database general error occurred: {:?} / {}",
            self.public, self.details
        )
    }
}

impl std::error::Error for SecretVaultDatabaseError {}

#[derive(Debug, Clone, Builder)]
pub struct SecretVaultDataConflictError {
    pub public: SecretVaultErrorPublicGenericDetails,
    pub details: String,
}

impl Display for SecretVaultDataConflictError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Database conflict error occurred: {:?} / {}",
            self.public, self.details
        )
    }
}

impl std::error::Error for SecretVaultDataConflictError {}

#[derive(Debug, Clone, Builder)]
pub struct SecretVaultDataNotFoundError {
    pub public: SecretVaultErrorPublicGenericDetails,
    pub data_detail_message: String,
}

impl Display for SecretVaultDataNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Data not found error occurred: {:?}", self.public)
    }
}

impl std::error::Error for SecretVaultDataNotFoundError {}

#[derive(Debug, PartialEq, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SecretVaultInvalidParametersPublicDetails {
    pub field: String,
    pub error: String,
}

#[derive(Debug, Clone, Builder)]
pub struct SecretVaultInvalidParametersError {
    pub public: SecretVaultInvalidParametersPublicDetails,
}

impl Display for SecretVaultInvalidParametersError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Data not found error occurred: {:?}", self.public)
    }
}

impl std::error::Error for SecretVaultInvalidParametersError {}

#[derive(Debug, PartialEq, Clone, Builder)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SecretVaultInvalidJsonErrorPublicDetails {
    pub code: String,
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SecretVaultNetworkError {
    pub public: SecretVaultErrorPublicGenericDetails,
    pub message: String,
}

impl Display for SecretVaultNetworkError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Network error: {:?} / {}", self.public, self.message)
    }
}

impl std::error::Error for SecretVaultNetworkError {}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SecretVaultEncryptionError {
    pub public: SecretVaultErrorPublicGenericDetails,
    pub message: String,
}

impl SecretVaultEncryptionError {
    pub fn create(code: &str, message: &str) -> SecretVaultError {
        SecretVaultError::EncryptionError(SecretVaultEncryptionError::new(
            SecretVaultErrorPublicGenericDetails::new(code.to_string()),
            message.to_string(),
        ))
    }
}

impl Display for SecretVaultEncryptionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "SecretVault encryption error: {:?} / {}",
            self.public, self.message
        )
    }
}

impl std::error::Error for SecretVaultEncryptionError {}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SecretVaultMemoryError {
    pub public: SecretVaultErrorPublicGenericDetails,
    pub message: String,
}

impl SecretVaultMemoryError {
    pub fn create(code: &str, message: &str) -> SecretVaultError {
        SecretVaultError::MemoryError(SecretVaultMemoryError::new(
            SecretVaultErrorPublicGenericDetails::new(code.to_string()),
            message.to_string(),
        ))
    }
}

impl Display for SecretVaultMemoryError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "SecretVault memory error: {:?} / {}",
            self.public, self.message
        )
    }
}

impl std::error::Error for SecretVaultMemoryError {}

#[cfg(gcloud)]
impl From<gcloud_sdk::error::Error> for SecretVaultError {
    fn from(e: gcloud_sdk::error::Error) -> Self {
        SecretVaultError::SystemError(SecretVaultSystemError::new(
            SecretVaultErrorPublicGenericDetails::new(format!("{:?}", e.kind())),
            format!("GCloud system error: {}", e),
        ))
    }
}

#[cfg(gcloud)]
impl From<tonic::Status> for SecretVaultError {
    fn from(status: tonic::Status) -> Self {
        match status.code() {
            tonic::Code::NotFound => {
                SecretVaultError::DataNotFoundError(SecretVaultDataNotFoundError::new(
                    SecretVaultErrorPublicGenericDetails::new(format!("{:?}", status.code())),
                    format!("{}", status),
                ))
            }
            tonic::Code::Unknown => check_hyper_errors(status),
            _ => SecretVaultError::SystemError(SecretVaultSystemError::new(
                SecretVaultErrorPublicGenericDetails::new(format!("{:?}", status.code())),
                format!("{}", status),
            )),
        }
    }
}