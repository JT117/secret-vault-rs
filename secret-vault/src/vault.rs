use crate::allocator::SecretVaultStoreValueAllocator;
use crate::encryption::SecretVaultEncryption;
use crate::secrets_source::SecretsSource;
use crate::vault_store::SecretVaultStore;
use crate::{
    SecretVaultRef, SecretVaultResult, SecretVaultSnapshot, SecretVaultView, SecretVaultViewer,
};
use secret_vault_value::SecretValue;
use tracing::*;

pub struct SecretVault<S, AR, E>
where
    S: SecretsSource,
    E: SecretVaultEncryption,
    AR: SecretVaultStoreValueAllocator,
{
    source: S,
    store: SecretVaultStore<AR, E>,
    refs: Vec<SecretVaultRef>,
}

impl<S, AR, E> SecretVault<S, AR, E>
where
    S: SecretsSource,
    E: SecretVaultEncryption,
    AR: SecretVaultStoreValueAllocator,
{
    pub fn new(source: S, store: SecretVaultStore<AR, E>) -> SecretVaultResult<Self> {
        Ok(Self {
            source,
            store,
            refs: Vec::new(),
        })
    }

    pub fn with_secrets_refs(&mut self, secret_refs: Vec<&SecretVaultRef>) -> &mut Self {
        self.refs = secret_refs.into_iter().cloned().collect();
        self
    }

    pub async fn refresh(&mut self) -> SecretVaultResult<&mut Self> {
        info!(
            "Refreshing secrets from the source: {}. Expected: {}. Required: {}",
            self.source.name(),
            self.refs.len(),
            self.refs
                .iter()
                .filter(|secret_ref| secret_ref.required)
                .count()
        );

        let mut secrets_map = self.source.get_secrets(&self.refs).await?;

        for (secret_ref, secret_value) in secrets_map.drain() {
            self.store.insert(secret_ref, &secret_value)?;
        }

        info!("Secret vault contains: {} secrets", self.store.len());

        Ok(self)
    }

    pub fn viewer(&self) -> SecretVaultViewer<AR, E> {
        SecretVaultViewer::new(&self.store)
    }

    pub fn snapshot(self) -> SecretVaultSnapshot<AR, E> {
        SecretVaultSnapshot::new(self.store)
    }
}

impl<S, AR, E> SecretVaultView for SecretVault<S, AR, E>
where
    S: SecretsSource,
    E: SecretVaultEncryption,
    AR: SecretVaultStoreValueAllocator,
{
    fn get_secret_by_ref(
        &self,
        secret_ref: &SecretVaultRef,
    ) -> SecretVaultResult<Option<SecretValue>> {
        self.store.get_secret(secret_ref)
    }
}