use actix_web::{test, App};
use actix_web::http::StatusCode;
use actix_web::dev::ServiceResponse;
use serde_json::Value;
use std::sync::Arc;

use api_gateway::register;
use api_gateway::schema::Schema;

async fn get_body(response: ServiceResponse) -> Value {
    let body = actix_web::test::read_body(response).await;
    serde_json::from_str(std::str::from_utf8(&body).expect("utf8 parse error")).expect("json parse error")
}

async fn call_request(schema: Arc<Schema>, payload: Value) -> ServiceResponse {
    let mut app = test::init_service(App::new()
        .configure(register(schema.clone()))).await;
    let request = test::TestRequest::post().uri("/graphql").set_json(&payload).to_request();

    test::call_service(&mut app, request).await
}

/// Calls the request and gets the status and response
pub async fn get_response(schema: Arc<Schema>, payload: Value) -> (StatusCode, Value) {
        let response = call_request(schema, payload).await;
        let status = response.status();
        let body = get_body(response).await;

        (status, body)
}
