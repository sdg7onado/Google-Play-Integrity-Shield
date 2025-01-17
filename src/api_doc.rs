use crate::handler::__path_health_check;
use crate::models::{GenericResponseObject, PlayIntegrityRequest, PlayIntegrityResponse};
use crate::validateintegritytoken::__path_validate_token;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(validate_token, health_check),
    components(schemas(PlayIntegrityRequest, PlayIntegrityResponse, GenericResponseObject))
)]
pub struct ApiDoc;
