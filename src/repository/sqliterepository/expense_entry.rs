use uuid::Uuid;

use crate::{
    domain::{cost_share::CostShare, expense_entry::ExpenseEntry},
    service::{
        application_error::{ApplicationError, ApplicationErrorType},
        expense_entry::{ExpenseEntryReadPort, ExpenseEntryWritePort},
    },
    test_util::test_utility::TEST_VALID_UUID,
};

pub struct ExpenseEntryReadSqliteRepository {}
pub struct ExpenseEntryWriteSqliteRepository {}

impl ExpenseEntryReadSqliteRepository {
    pub fn new() -> Self {
        ExpenseEntryReadSqliteRepository {}
    }
}

impl ExpenseEntryWriteSqliteRepository {
    pub fn new() -> Self {
        ExpenseEntryWriteSqliteRepository {}
    }
}

impl ExpenseEntryReadPort for ExpenseEntryReadSqliteRepository {
    fn get(&self, id: Uuid) -> Result<ExpenseEntry, ApplicationError> {
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

impl ExpenseEntryWritePort for ExpenseEntryWriteSqliteRepository {
    fn insert(&self, entry: ExpenseEntry) -> Result<ExpenseEntry, ApplicationError> {
        Ok(entry)
    }

    fn update(&self, id: Uuid, entry: ExpenseEntry) -> Result<ExpenseEntry, ApplicationError> {
        todo!()
    }

    fn delete(&self, id: Uuid) -> Result<(), ApplicationError> {
        match id {
            id if id == TEST_VALID_UUID => Ok(()),
            _ => Err(ApplicationError {
                error_type: ApplicationErrorType::NotFound,
                message: String::from("Expense entry not found."),
            }),
        }
    }
}
