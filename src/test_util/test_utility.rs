//#![cfg(any(test, feature = "test-utils"))]
pub const TEST_VALID_UUID: uuid::Uuid = uuid::uuid!("123e4567-e89b-12d3-a456-426614174000");
pub const TEST_INVALID_UUID: uuid::Uuid = uuid::uuid!("123e4567-e89b-12d3-a456-426614174001");