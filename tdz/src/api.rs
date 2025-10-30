use std::fs;
use serde_json;
use crate::error::{Result, TodoziError};
use crate::models::{ApiKey, ApiKeyCollection};
use crate::storage;
pub struct ApiKeyManager {
    pub collection: ApiKeyCollection,
}
impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            collection: ApiKeyCollection::new(),
        }
    }
}
pub fn save_api_key_collection(collection: &ApiKeyCollection) -> Result<()> {
    let storage_dir = storage::get_storage_dir()?;
    let api_dir = storage_dir.join("api");
    fs::create_dir_all(&api_dir)?;
    let file_path = api_dir.join("api_keys.json");
    let content = serde_json::to_string_pretty(collection)?;
    fs::write(file_path, content)?;
    Ok(())
}
pub fn load_api_key_collection() -> Result<ApiKeyCollection> {
    let storage_dir = storage::get_storage_dir()?;
    let file_path = storage_dir.join("api").join("api_keys.json");
    if !file_path.exists() {
        return Ok(ApiKeyCollection::new());
    }
    let content = fs::read_to_string(file_path)?;
    let collection: ApiKeyCollection = serde_json::from_str(&content)?;
    Ok(collection)
}
pub fn create_api_key() -> Result<ApiKey> {
    let api_key = ApiKey::new();
    let mut collection = load_api_key_collection()?;
    collection.add_key(api_key.clone());
    save_api_key_collection(&collection)?;
    Ok(api_key)
}
pub fn create_api_key_with_user_id(user_id: String) -> Result<ApiKey> {
    let api_key = ApiKey::with_user_id(user_id);
    let mut collection = load_api_key_collection()?;
    collection.add_key(api_key.clone());
    save_api_key_collection(&collection)?;
    Ok(api_key)
}
pub fn get_api_key(user_id: &str) -> Result<ApiKey> {
    let collection = load_api_key_collection()?;
    collection
        .get_key(user_id)
        .ok_or_else(|| TodoziError::ValidationError {
            message: format!("API key not found: {}", user_id),
        })
        .map(|key| (*key).clone())
}
pub fn get_api_key_by_public(public_key: &str) -> Result<ApiKey> {
    let collection = load_api_key_collection()?;
    collection
        .get_key_by_public(public_key)
        .ok_or_else(|| TodoziError::ValidationError {
            message: format!("API key not found for public key: {}", public_key),
        })
        .map(|key| (*key).clone())
}
pub fn list_api_keys() -> Result<Vec<ApiKey>> {
    let collection = load_api_key_collection()?;
    Ok(collection.get_all_keys().into_iter().cloned().collect())
}
pub fn list_active_api_keys() -> Result<Vec<ApiKey>> {
    let collection = load_api_key_collection()?;
    Ok(collection.get_active_keys().into_iter().cloned().collect())
}
pub fn check_api_key_auth(
    public_key: &str,
    private_key: Option<&str>,
) -> Result<(String, bool)> {
    let collection = load_api_key_collection()?;
    let api_key = collection
        .get_key_by_public(public_key)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Invalid API key".to_string(),
        })?;
    let is_admin = if let Some(priv_key) = private_key {
        api_key.is_admin(public_key, priv_key)
    } else {
        api_key.matches(public_key, None)
    };
    Ok((api_key.user_id.clone(), is_admin))
}
pub fn deactivate_api_key(user_id: &str) -> Result<()> {
    let mut collection = load_api_key_collection()?;
    if !collection.deactivate_key(user_id) {
        return Err(TodoziError::ValidationError {
            message: format!("API key not found: {}", user_id),
        });
    }
    save_api_key_collection(&collection)?;
    Ok(())
}
pub fn activate_api_key(user_id: &str) -> Result<()> {
    let mut collection = load_api_key_collection()?;
    if !collection.activate_key(user_id) {
        return Err(TodoziError::ValidationError {
            message: format!("API key not found: {}", user_id),
        });
    }
    save_api_key_collection(&collection)?;
    Ok(())
}
pub fn remove_api_key(user_id: &str) -> Result<ApiKey> {
    let mut collection = load_api_key_collection()?;
    let key = collection
        .remove_key(user_id)
        .ok_or_else(|| TodoziError::ValidationError {
            message: format!("API key not found: {}", user_id),
        })?;
    save_api_key_collection(&collection)?;
    Ok(key)
}