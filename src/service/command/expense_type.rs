use uuid::Uuid;

use crate::domain::expense_type::ExpenseType;
use crate::service::application_error::ApplicationError;
use crate::service::expense_type::{ExpenseTypeNew, ExpenseTypeService};

impl ExpenseTypeService {
    pub fn create(&self, dto: ExpenseTypeNew) -> Result<ExpenseType, ApplicationError> {
        let expense_type = ExpenseType::try_from(dto)?;
        self.write_repo.insert(expense_type)
    }

    pub fn update(&self, _id: Uuid, _dto: ExpenseTypeNew) -> Result<ExpenseType, ApplicationError> {
        todo!()
    }

    pub fn delete(&self, id: Uuid) -> Result<(), ApplicationError> {
        self.write_repo.delete(id)
    }
}
