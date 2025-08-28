use uuid::Uuid;

use crate::{domain::expense_entry::ExpenseEntry, util::cost_share::CostShare, service::application_error::{ApplicationError, ApplicationErrorType}};

const TEST_EXPENSE_ENTRY_ID: &str = "123e4567-e89b-12d3-a456-426614174000";

pub fn get(id: Uuid) -> Result<ExpenseEntry, ApplicationError> {
    let test_id = Uuid::parse_str(TEST_EXPENSE_ENTRY_ID).unwrap();
    match id {
        id if id == test_id => Ok(
            ExpenseEntry::new(vec![CostShare{cost_bearer_id: Uuid::new_v4(), amount:12.5}], Uuid::new_v4(), String::from("Some Description")).unwrap()),
        _ => Err(ApplicationError { error_type: ApplicationErrorType::NotFound, message: String::from("Expense entry not found.") })
    }
}
