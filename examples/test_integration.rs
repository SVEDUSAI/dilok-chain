use dilok_did::{DilokDid, StorageConfig};
use dilok_did::core::types::{UserInfo, Gender, IdProof, IdProofType};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting DID system integration test...");

    // Initialize the DID system with test configuration
    let config = StorageConfig {
        onchain_storage_path: "./test_data/onchain".to_string(),
        offchain_storage_path: "./test_data/offchain".to_string(),
        encryption_key: "test-encryption-key".to_string(),
    };

    println!("Initializing DID system...");
    let did_system = DilokDid::new(config)?;

    // Create test user information
    let user_info = UserInfo {
        id: uuid::Uuid::new_v4(),
        did: format!("did:dilok:test:{}", uuid::Uuid::new_v4()),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        date_of_birth: Utc::now(),
        gender: Gender::Other,
        phone_number: "+1234567890".to_string(),
        email: "test@example.com".to_string(),
        id_proof: IdProof {
            proof_type: IdProofType::Passport,
            number: "TEST123456".to_string(),
            issuing_authority: "Test Authority".to_string(),
            issued_date: Utc::now(),
            expiry_date: Utc::now(),
        },
        face_hash: "".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    println!("Registering test user...");
    // Register user with face recognition (using camera index 0)
    did_system.register_user(user_info.clone(), 0).await?;

    println!("Verifying test user...");
    // Verify the registered user
    let is_verified = did_system.verify_user(&user_info.did, 0).await?;
    println!("User verification result: {}", is_verified);

    // Retrieve and verify user information
    println!("Retrieving user information...");
    let retrieved_user = did_system.get_user_info(&user_info.did).await?;
    println!("Retrieved user: {:#?}", retrieved_user);

    println!("Integration test completed successfully!");
    Ok(())
} 