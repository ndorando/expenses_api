use uuid::Uuid;

use crate::{domain::{cost_share::CostShare, expense_entry::ExpenseEntry}, service::{application_error::{ApplicationError, ApplicationErrorType}, expense_entry::ExpenseEntryReadPort}, test_util::test_utility::TEST_VALID_UUID};


pub struct ExpenseEntryReadSqliteRepositry {
}

impl ExpenseEntryReadSqliteRepositry {
    pub fn new() -> Self {
        ExpenseEntryReadSqliteRepositry {  }
    }
}

impl ExpenseEntryReadPort for ExpenseEntryReadSqliteRepositry {
    fn get(&self, id:Uuid) -> Result<ExpenseEntry, ApplicationError> {
        match id {
        id if id == TEST_VALID_UUID => Ok(ExpenseEntry::new(
            vec![CostShare {
                cost_bearer_id: Uuid::new_v4(),
                amount: 12.5,
            }],
            Uuid::new_v4(),
            String::from("Some Description"),
            None,
        )
        .unwrap()),
        _ => Err(ApplicationError {
            error_type: ApplicationErrorType::NotFound,
            message: String::from("Expense entry not found."),
        }),
    }
    }
}