use uuid::Uuid;

use crate::{
    domain::expense_type::ExpenseType,
    service::{
        application_error::{ApplicationError, ApplicationErrorType},
        expense_type::{ExpenseTypeReadPort, ExpenseTypeWritePort},
    },
    test_util::test_utility::TEST_VALID_UUID,
};

pub struct ExpenseTypeReadSqliteRepository {}
pub struct ExpenseTypeWriteSqliteRepository {}

impl ExpenseTypeReadSqliteRepository {
    pub fn new() -> Self {
        ExpenseTypeReadSqliteRepository {}
    }
}

impl ExpenseTypeWriteSqliteRepository {
    pub fn new() -> Self {
        ExpenseTypeWriteSqliteRepository {}
    }
}

impl ExpenseTypeReadPort for ExpenseTypeReadSqliteRepository {
    fn get(&self, id: Uuid) -> Result<ExpenseType, ApplicationError> {
        match id {
            id if id == TEST_VALID_UUID => Ok(ExpenseType::new(
                String::from("Food"),
                String::from("Expenses related to food and dining"),
            )
            .unwrap()),
            _ => Err(ApplicationError {
                error_type: ApplicationErrorType::NotFound,
                message: String::from("Expense type not found."),
            }),
        }
    }
}

impl ExpenseTypeWritePort for ExpenseTypeWriteSqliteRepository {
    fn insert(&self, entry: ExpenseType) -> Result<ExpenseType, ApplicationError> {
        Ok(entry)
    }

    fn update(&self, _id: Uuid, _entry: ExpenseType) -> Result<ExpenseType, ApplicationError> {
        todo!()
    }

    fn delete(&self, id: Uuid) -> Result<(), ApplicationError> {
        match id {
            id if id == TEST_VALID_UUID => Ok(()),
            _ => Err(ApplicationError {
                error_type: ApplicationErrorType::NotFound,
                message: String::from("Expense type not found."),
            }),
        }
    }
}
