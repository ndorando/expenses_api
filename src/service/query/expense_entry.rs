use uuid::Uuid;

use crate::domain::expense_entry::ExpenseEntry;
use crate::domain::cost_share::CostShare;
use crate::service::application_error::{ApplicationError, ApplicationErrorType};

use crate::test_util::test_utility::TEST_VALID_UUID;

pub fn get(id: Uuid) -> Result<ExpenseEntry, ApplicationError> {
    match id {
        id if id == TEST_VALID_UUID => Ok(
            ExpenseEntry::new(vec![CostShare{cost_bearer_id: Uuid::new_v4(), amount:12.5}], Uuid::new_v4(), String::from("Some Description"), None).unwrap()),
        _ => Err(ApplicationError { error_type: ApplicationErrorType::NotFound, message: String::from("Expense entry not found.") })
    }
}
