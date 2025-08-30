use chrono::{DateTime, Utc};
use uuid::Uuid;
use thiserror::Error;

use crate::domain::cost_share::CostShare;

// validated and guaranteed to be correct data
#[derive(serde::Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct ExpenseEntry {
    id: Uuid,
    expense_date: DateTime<Utc>,
    cost_shares: Vec<CostShare>,
    expense_type: Uuid,
    description: String,
}       

#[derive(Debug, Error)]
pub enum ExpenseEntryValidationError {
     #[error("Expense Entry Validation failed: No cost shares provided.")]
    MissingCostShares,

    #[error("Expense Entry Validation failed: Cost shares malformed.")]
    MalformedCostShares,

    #[error("Expense Entry Validation failed: Invalid cost bearer Id: {0}.")]
    InvalidCostBearerId(Uuid),

    #[error("Expense Entry Validation failed: Duplicate cost bearer Id: {0}.")]
    DuplicateCostBearerIds(Uuid),

    #[error("Expense Entry Validation failed: Missing expense type.")]
    MissingExpenseType,

    #[error("Expense Entry Validation failed: Invalid expense type Id {0}.")]
    InvalidExpenseTypeId(Uuid),

    #[error("Expense Entry Validation failed: Description is empty or whitespace.")]
    MissingDescription,
}

impl ExpenseEntry {
    pub fn new(cost_shares: Vec<CostShare>, expense_type: Uuid, description: String, expense_date: impl Into<Option<DateTime<Utc>>>) -> Result<Self, ExpenseEntryValidationError> {

        // validate cost shares
        if cost_shares.is_empty() {
            return Err(ExpenseEntryValidationError::MissingCostShares);
        }
    
        let mut seen = std::collections::HashSet::new();
        for share in &cost_shares {
            // check for duplicate cost bearer ids
            if !seen.insert(share.cost_bearer_id) {
                return Err(ExpenseEntryValidationError::DuplicateCostBearerIds(share.cost_bearer_id));
            }
            // check for it never being 0.0
            if share.amount == 0.0f64 || share.cost_bearer_id.is_nil() {
                return Err(ExpenseEntryValidationError::MalformedCostShares);
            }
            if false {
                todo!("Implementation of cost share validation for ExpenseEntry must still happen once we have a DB.");
            }
        }

        // validate expense type
        if expense_type.is_nil() {
            return Err(ExpenseEntryValidationError::MissingExpenseType);
        }

        if false {
            todo!("Implementation of expense type validation for ExpenseEntry must still happen once we have a DB.");
        }

        // validate description
        if description.trim().is_empty() {
            return Err(ExpenseEntryValidationError::MissingDescription);
        }

        Ok(Self {
            id: Uuid::new_v4(),
            expense_date: expense_date.into().unwrap_or_else(chrono::Utc::now),
            cost_shares,
            expense_type,
            description,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn expense_date(&self) -> DateTime<Utc> {
        self.expense_date
    }
    pub fn cost_shares(&self) -> &[CostShare] {
        &self.cost_shares
    }
    pub fn expense_type(&self) -> Uuid {
        self.expense_type
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}