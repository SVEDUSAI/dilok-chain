pub mod core;
pub mod face_recognition;
pub mod storage;
pub mod api;

use core::types::{UserInfo, FaceData, StorageConfig};
use face_recognition::FaceRecognition;
use storage::{OnChainStorage, OffChainStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DilokDidError {
    #[error("Face recognition error: {0}")]
    FaceRecognitionError(#[from] face_recognition::FaceRecognitionError),
    #[error("Storage error: {0}")]
    StorageError(#[from] storage::StorageError),
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

#[derive(Clone)]
pub struct DilokDid {
    face_recognition: FaceRecognition,
    onchain_storage: OnChainStorage,
    offchain_storage: OffChainStorage,
}

impl DilokDid {
    pub fn new(config: StorageConfig) -> Result<Self, DilokDidError> {
        let face_recognition = FaceRecognition::new()
            .map_err(DilokDidError::FaceRecognitionError)?;

        let onchain_storage = OnChainStorage::new(std::path::Path::new(&config.onchain_storage_path))
            .map_err(DilokDidError::StorageError)?;

        let offchain_storage = OffChainStorage::new(&config)
            .map_err(DilokDidError::StorageError)?;

        Ok(Self {
            face_recognition,
            onchain_storage,
            offchain_storage,
        })
    }

    pub async fn register_user(&self, user_info: UserInfo, camera_index: i32) -> Result<(), DilokDidError> {
        // Capture and process face data
        let face_data = self.face_recognition.capture_face(camera_index)
            .map_err(DilokDidError::FaceRecognitionError)?;

        // Store face data off-chain
        self.offchain_storage.store_face_data(&face_data)
            .map_err(DilokDidError::StorageError)?;

        // Store user info and face hash on-chain
        self.onchain_storage.store_user_info(&user_info)
            .map_err(DilokDidError::StorageError)?;
        self.onchain_storage.store_face_hash(&user_info.id.to_string(), &face_data.face_hash)
            .map_err(DilokDidError::StorageError)?;

        Ok(())
    }

    pub async fn verify_user(&self, user_id: &str, camera_index: i32) -> Result<bool, DilokDidError> {
        // Get face hash from on-chain storage
        let face_hash = self.onchain_storage.get_face_hash(user_id)
            .map_err(DilokDidError::StorageError)?;

        // Get face data from off-chain storage
        let face_data = self.offchain_storage.get_face_data(user_id)
            .map_err(DilokDidError::StorageError)?;

        // Verify face
        let result = self.face_recognition.verify_face(&face_data, camera_index)
            .map_err(DilokDidError::FaceRecognitionError)?;

        Ok(result.is_verified)
    }

    pub async fn get_user_info(&self, user_id: &str) -> Result<UserInfo, DilokDidError> {
        self.onchain_storage.get_user_info(user_id)
            .map_err(DilokDidError::StorageError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_registration() {
        let config = StorageConfig {
            onchain_storage_path: "test_onchain".to_string(),
            offchain_storage_path: "test_offchain".to_string(),
            encryption_key: "test_key".to_string(),
        };

        let did_system = DilokDid::new(config).unwrap();
        // Add more test cases here
    }
} 