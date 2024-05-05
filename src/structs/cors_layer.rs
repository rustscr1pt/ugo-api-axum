use axum::http::{header, HeaderValue, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};
use crate::structs::constants::CORS_ROUTE;

// For setting up cors layer in the future.
// https://github.com/tower-rs/tower-http/issues/194
pub fn get_cors_layer() -> CorsLayer {
    return
        CorsLayer::new()
            .allow_origin(AllowOrigin::exact(CORS_ROUTE.parse().unwrap()))
            .allow_headers(vec![
                header::CONTENT_TYPE,
            ])
            .allow_methods(vec![
                Method::GET,
                Method::POST,
            ])
}