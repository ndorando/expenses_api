use uuid::Uuid;

use crate::domain::cost_bearer::CostBearer;
use crate::service::application_error::{ApplicationError, ApplicationErrorType};
use crate::service::cost_bearer::CostBearerNew;

use crate::test_util::test_utility::TEST_VALID_UUID;

pub fn create(dto: CostBearerNew) -> Result<CostBearer, ApplicationError> {
    let entry = CostBearer::try_from(dto)?;

    // todo - save to DB
    
    Ok(entry)
}

pub fn update(id: Uuid, dto: CostBearerNew) -> Result<CostBearer, ApplicationError>  {
    todo!()
}

pub fn delete(id: Uuid) -> Result<(), ApplicationError> {
    match id {
        id if id == TEST_VALID_UUID => Ok(()),
        _ => Err(ApplicationError { error_type: ApplicationErrorType::NotFound, message: String::from("Cost Bearer not found.") })
    }
}
