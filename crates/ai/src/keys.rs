use crate::error::{AiError, Result};

const SERVICE: &str = "com.vspat.stark";

/// Store an API key in the OS credential store.
/// Never in source, never in localStorage, never in the repo.
pub fn store_key(provider: &str, key: &str) -> Result<()> {
    let entry = keyring::Entry::new(SERVICE, provider)
        .map_err(|e| AiError::Provider(format!("keyring: {e}")))?;
    entry
        .set_password(key)
        .map_err(|e| AiError::Provider(format!("keyring: {e}")))
}

pub fn load_key(provider: &str) -> Result<Option<String>> {
    let entry = keyring::Entry::new(SERVICE, provider)
        .map_err(|e| AiError::Provider(format!("keyring: {e}")))?;
    match entry.get_password() {
        Ok(k) => Ok(Some(k)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(AiError::Provider(format!("keyring: {e}"))),
    }
}

pub fn delete_key(provider: &str) -> Result<()> {
    let entry = keyring::Entry::new(SERVICE, provider)
        .map_err(|e| AiError::Provider(format!("keyring: {e}")))?;
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(AiError::Provider(format!("keyring: {e}"))),
    }
}