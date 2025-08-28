/*  ApplicationError.rs
 *  
 *  Internal Error Representation that can be extended for logging or similar tasks.
 *  Can include internal logic that is only used within this application.
 *  Can be returned as an error response for axum via the IntoResponse trait.
 */

pub enum ApplicationErrorType {
    NotFound,
    ValidationFailed,
    // this can be extended
}

pub struct ApplicationError {
    pub error_type: ApplicationErrorType,
    pub message: String,
    // this can be extended
}