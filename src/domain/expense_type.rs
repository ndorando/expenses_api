use uuid::Uuid;
use thiserror::Error;

// validated and guaranteed to be correct data
#[derive(serde::Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct ExpenseType {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(Debug, Error)]
pub enum ExpenseTypeValidationError {
    #[error("Expense Type Validation failed: Name is empty or whitespace.")]
    MissingName,
    #[error("Expense Type Validation failed: Description is empty or whitespace.")]
    MissingDescription,
    #[error("Expense Type Validation failed: Name already in use.")]
    DuplicateName,
}

impl ExpenseType {
    pub fn new(name: String, description: String) -> Result<Self, ExpenseTypeValidationError> {
        // validate that the name is not empty
        if name.trim().is_empty() {
            return Err(ExpenseTypeValidationError::MissingName);
        }

        // validate that the description is not empty
        if description.trim().is_empty() {
            return Err(ExpenseTypeValidationError::MissingDescription);
        }

        // validate that the name is unique (database-dependent validation)
        if false {
            todo!("Implementation of expense type name uniqueness validation must still happen once we have a DB.");
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}      