// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct HealthCheckRequest {}

// #[allow(non_snake_case)]
// #[derive(Default,Serialize)]
// pub struct HealthCheckResponse{
// pub healthy: bool,
// }

#[allow(unused_imports)]
use crate::dab::health_check::get::HealthCheckRequest;
use crate::dab::health_check::get::HealthCheckResponse;
use serde_json::json;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn process(_packet: String) -> Result<String, String> {
    let mut ResponseOperator = HealthCheckResponse::default();
    // *** Fill in the fields of the struct HealthCheckResponse here ***
    ResponseOperator.healthy = true;
    // *******************************************************************
    let mut ResponseOperator_json = json!(ResponseOperator);
    ResponseOperator_json["status"] = json!(200);
    Ok(serde_json::to_string(&ResponseOperator_json).unwrap())
}
