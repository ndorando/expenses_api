use axum::{extract::Path, http::StatusCode, Json};
use uuid::Uuid;

use crate::domain::expense_type::ExpenseType;
use crate::service::application_error::ApplicationError;
use crate::service::expense_type::ExpenseTypeNew;

pub async fn expense_type_post(expense_type: Json<ExpenseTypeNew>) -> Result<Json<ExpenseType>, ApplicationError> {
    let new_expense_type_dto: ExpenseTypeNew = expense_type.0;
    let created_expense_type = crate::service::command::expense_type::create(new_expense_type_dto)?;
    Ok(Json(created_expense_type))
}

pub async fn expense_type_update(Path(id): Path<Uuid>, expense_type: Json<ExpenseTypeNew>) -> Result<Json<ExpenseType>, ApplicationError> {
    let update_expense_type_dto: ExpenseTypeNew = expense_type.0;
    let updated_expense_type = crate::service::command::expense_type::update(id, update_expense_type_dto)?;
    Ok(Json(updated_expense_type))
}

pub async fn expense_type_delete(Path(id): Path<Uuid>) -> Result<StatusCode, ApplicationError> {
    crate::service::command::expense_type::delete(id)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn expense_type_get(Path(id): Path<Uuid>) -> Result<Json<ExpenseType>, ApplicationError> {
    let found_expense_type = crate::service::query::expense_type::get(id)?;
    Ok(Json(found_expense_type))
}
