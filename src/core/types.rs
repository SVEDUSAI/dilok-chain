use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub did: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: DateTime<Utc>,
    pub gender: Gender,
    pub phone_number: String,
    pub email: String,
    pub id_proof: IdProof,
    pub face_hash: String,  // Hash of face recognition data
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdProof {
    pub proof_type: IdProofType,
    pub number: String,
    pub issuing_authority: String,
    pub issued_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdProofType {
    Passport,
    DriverLicense,
    NationalId,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    pub user_id: Uuid,
    pub face_hash: String,
    pub face_data: Vec<u8>,  // Encrypted face recognition data
    pub angles: Vec<FaceAngle>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceAngle {
    pub angle: f32,
    pub quality_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub is_verified: bool,
    pub confidence_score: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub onchain_storage_path: String,
    pub offchain_storage_path: String,
    pub encryption_key: String,
} 