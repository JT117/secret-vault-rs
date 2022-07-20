use chrono::prelude::*;
use rsb_derive::*;
use rvstruct::*;
use secret_vault_value::SecretValue;

#[derive(Debug, Clone, Eq, PartialEq, Hash, ValueStruct)]
pub struct SecretName(String);

#[derive(Debug, Clone, Eq, PartialEq, Hash, ValueStruct)]
pub struct SecretVersion(String);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Builder)]
pub struct SecretVaultRef {
    pub secret_name: SecretName,
    pub secret_version: Option<SecretVersion>,

    #[default = "true"]
    pub required: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Builder)]
pub struct SecretMetadataLabel {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Builder)]
pub struct SecretMetadata {
    pub labels: Option<Vec<SecretMetadataLabel>>,
    pub description: Option<String>,
    pub expire_at: Option<DateTime<Utc>>,
    pub version: Option<SecretVersion>,
}

#[derive(Debug, Clone, Eq, PartialEq, Builder)]
pub struct Secret {
    pub value: SecretValue,
    pub metadata: SecretMetadata,
}
