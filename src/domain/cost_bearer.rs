use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CostBearer {
    id: Uuid,
    name: String,
    exists_from: DateTime<Utc>,
    exists_to: Option<DateTime<Utc>>
}

#[derive(Debug)]
pub enum CostBearerValidationError {
    MissingName,
    InvalidDate,
}

impl CostBearer {
    pub fn new (name: String, exists_from: DateTime<Utc>, exists_to: Option<DateTime<Utc>>) -> Result<Self, CostBearerValidationError> {

        // validate that the name is no empty
        if name.trim().is_empty() {
            return Err(CostBearerValidationError::MissingName);
        }

        // verify that exists_to is not before exists_from
        if let Some(to) = exists_to {
            if to <= exists_from {
                return Err(CostBearerValidationError::InvalidDate);
            }
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name: name,
            exists_from: exists_from,
            exists_to: exists_to,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn exists_from(&self) -> DateTime<Utc> {
        self.exists_from
    }

    pub fn exists_to(&self) -> Option<DateTime<Utc>> {
        self.exists_to
    }
}