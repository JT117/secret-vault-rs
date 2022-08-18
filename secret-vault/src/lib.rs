//! # Secret Vault for Rust
//!
//! Library provides a secure memory-backed storage of secrets coming to your application
//! from external sources:
//!
//! - Google Cloud Secret Manager
//! - Amazon Secrets Manager
//! - Environment variables
//! - Files source (mostly designed to read K8S secrets mounted as files)
//!
//! ## Features
//! - Caching registered secrets in memory from sources;
//! - Extensible and strongly typed API to be able to implement any kind of sources;
//! - Memory encryption using AEAD cryptography (optional);
//! - Memory encryption using Google/AWS KMS envelope encryption (https://cloud.google.com/kms/docs/envelope-encryption) (optional);
//! - Automatic refresh secrets from the sources support (optional);
//! - Multi-sources support;
//!
//! ```rust,ignore
//!
//!     // Describing secrets and marking them non-required
//!    // since this is only example and they don't exist in your project
//!    let secret_ref1 = SecretVaultRef::new("test-secret-xRnpry".into())
//!        .with_required(false)
//!        .with_secret_version("AWSCURRENT".into());
//!    let secret_ref2 = SecretVaultRef::new("another-secret-222222".into()).with_required(false);
//!
//!    // Building the vault
//!    let mut vault = SecretVaultBuilder::with_source(
//!        aws::AwsSecretManagerSource::new(&config_env_var("ACCOUNT_ID")?, None).await?,
//!    )
//!    .with_encryption(ring_encryption::SecretVaultRingAeadEncryption::new()?)
//!    .build()?;
//!
//!    // Registering your secrets and receiving them from source
//!    vault
//!        .register_secrets_refs(vec![&secret_ref1, &secret_ref2])
//!        .refresh()
//!        .await?;
//!
//!    // Reading the secret
//!    let secret_value: Option<Secret> = vault.get_secret_by_ref(&secret_ref1).await?;
//!
//!    // Using the Viewer API to share only methods able to read secrets
//!    let vault_viewer = vault.viewer();
//!    vault_viewer.get_secret_by_ref(&secret_ref2).await?;
//! ```
//!
//! ## Examples, more detail docs and security considerations and benchmarks:
//! Available on github: https://github.com/abdolence/secret-vault-rs
//!
//! ```

#![allow(unused_parens, clippy::new_without_default, clippy::needless_update)]
#![forbid(unsafe_code)]

mod encryption;
pub use encryption::*;

pub mod errors;
mod secrets_source;
pub use secrets_source::*;

mod simple_sources;
pub use simple_sources::*;

mod vault_store;

mod common_types;
pub use common_types::*;

#[cfg(feature = "ring-aead-encryption")]
pub mod ring_encryption;

#[cfg(feature = "gcp")]
pub mod gcp;

#[cfg(feature = "aws")]
pub mod aws;

pub type SecretVaultResult<T> = std::result::Result<T, errors::SecretVaultError>;

mod vault;
pub use vault::*;

mod vault_builder;
pub use vault_builder::SecretVaultBuilder;

mod vault_viewer;
pub use vault_viewer::*;

mod vault_auto_refresher;
pub use vault_auto_refresher::*;

mod multiple_sources;
pub use multiple_sources::*;
