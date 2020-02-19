use actix_http::Request;
use actix_web::{test, App};
use actix_web::http::StatusCode;
use actix_web::dev::ServiceResponse;
use serde::de::DeserializeOwned;

use players_api::register;
use players_api::PgPool;

// Re-export the db connection
pub mod db_connection;

async fn get_body<T>(response: ServiceResponse) -> T 
    where T: DeserializeOwned {
    let body = actix_web::test::read_body(response).await;
    let body = std::str::from_utf8(&body).expect("utf8 parse error");
    serde_json::from_str(&body).unwrap()
}

async fn call_request(db_pool: &PgPool, request: Request) -> ServiceResponse {
    let mut app = test::init_service(App::new().configure(register(db_pool.clone()))).await;

    test::call_service(&mut app, request).await
}

/// Calls the request and gets the status and response
pub async fn get_response<T>(db_pool: &PgPool, request: Request) -> (StatusCode, T) 
    where T: DeserializeOwned {
        let response = call_request(db_pool, request).await;
        let status = response.status();
        let body = get_body(response).await;

        (status, body)
}

/// Calls the request and just returns the status. For requests
/// that do not have any response body so we don't try to deserialize it
pub async fn get_status(db_pool: &PgPool, request: Request) -> StatusCode {
    let response = call_request(db_pool, request).await;
    response.status()
}
