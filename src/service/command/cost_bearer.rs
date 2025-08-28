use uuid::Uuid;

use crate::{domain::cost_bearer::CostBearer, service::{application_error::{ApplicationError, ApplicationErrorType}, cost_bearer::CostBearerNew}};

const TEST_COST_BEARER_ID: &str = "123e4567-e89b-12d3-a456-426614174000";

pub fn create(dto: CostBearerNew) -> Result<CostBearer, ApplicationError> {
    let entry = CostBearer::try_from(dto)?;

    // todo - save to DB
    
    Ok(entry)
}

pub fn update(id: Uuid, dto: CostBearerNew) -> Result<CostBearer, ApplicationError>  {
    todo!()
}

pub fn delete(id: Uuid) -> Result<(), ApplicationError> {
    let test_id = Uuid::parse_str(TEST_COST_BEARER_ID).unwrap();
    match id {
        id if id == test_id => Ok(()),
        _ => Err(ApplicationError { error_type: ApplicationErrorType::NotFound, message: String::from("Cost Bearer not found.") })
    }
}
