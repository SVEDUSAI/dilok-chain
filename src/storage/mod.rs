use std::path::Path;
use std::sync::Arc;
use rocksdb::DB;
use crate::core::types::{UserInfo, FaceData, StorageConfig};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

#[derive(Clone)]
pub struct OnChainStorage {
    db: Arc<DB>,
}

#[derive(Clone)]
pub struct OffChainStorage {
    db: Arc<DB>,
    encryption_key: Vec<u8>,
}

impl OnChainStorage {
    pub fn new(path: &Path) -> Result<Self, StorageError> {
        let db = DB::open_default(path)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        Ok(Self { db: Arc::new(db) })
    }

    pub fn store_user_info(&self, user_info: &UserInfo) -> Result<(), StorageError> {
        let key = format!("user:{}", user_info.id);
        let value = serde_json::to_vec(user_info)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        
        self.db.put(key.as_bytes(), value)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub fn get_user_info(&self, user_id: &str) -> Result<UserInfo, StorageError> {
        let key = format!("user:{}", user_id);
        let value = self.db.get(key.as_bytes())
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?
            .ok_or_else(|| StorageError::NotFound(format!("User {} not found", user_id)))?;
        
        serde_json::from_slice(&value)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))
    }

    pub fn store_face_hash(&self, user_id: &str, face_hash: &str) -> Result<(), StorageError> {
        let key = format!("face_hash:{}", user_id);
        self.db.put(key.as_bytes(), face_hash.as_bytes())
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub fn get_face_hash(&self, user_id: &str) -> Result<String, StorageError> {
        let key = format!("face_hash:{}", user_id);
        let value = self.db.get(key.as_bytes())
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?
            .ok_or_else(|| StorageError::NotFound(format!("Face hash for user {} not found", user_id)))?;
        
        String::from_utf8(value)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))
    }
}

impl OffChainStorage {
    pub fn new(config: &StorageConfig) -> Result<Self, StorageError> {
        let db = DB::open_default(&config.offchain_storage_path)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        
        Ok(Self {
            db: Arc::new(db),
            encryption_key: config.encryption_key.as_bytes().to_vec(),
        })
    }

    pub fn store_face_data(&self, face_data: &FaceData) -> Result<(), StorageError> {
        let key = format!("face_data:{}", face_data.user_id);
        let value = serde_json::to_vec(face_data)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        
        // Encrypt the face data before storing
        let encrypted_value = self.encrypt_data(&value)?;
        
        self.db.put(key.as_bytes(), encrypted_value)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub fn get_face_data(&self, user_id: &str) -> Result<FaceData, StorageError> {
        let key = format!("face_data:{}", user_id);
        let encrypted_value = self.db.get(key.as_bytes())
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?
            .ok_or_else(|| StorageError::NotFound(format!("Face data for user {} not found", user_id)))?;
        
        // Decrypt the face data
        let decrypted_value = self.decrypt_data(&encrypted_value)?;
        
        serde_json::from_slice(&decrypted_value)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))
    }

    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        // TODO: Implement proper encryption using the encryption_key
        // For now, just return the data as is
        Ok(data.to_vec())
    }

    fn decrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        // TODO: Implement proper decryption using the encryption_key
        // For now, just return the data as is
        Ok(data.to_vec())
    }
} 