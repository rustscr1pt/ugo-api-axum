use axum::http::{header, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};

// For setting up cors layer in the future.
// https://github.com/tower-rs/tower-http/issues/194

// More info about setting up CORS
// https://docs.rs/tower-http/0.5.2/tower_http/cors/struct.AllowOrigin.html
// https://docs.rs/tower-http/0.5.2/tower_http/cors/struct.CorsLayer.html#method.allow_origin


// pub fn get_cors_layer() -> CorsLayer {
//     return
//         CorsLayer::new()
//             .allow_origin(AllowOrigin::exact(CORS_ROUTE.parse().unwrap()))
//             .allow_headers(vec![
//                 header::CONTENT_TYPE,
//             ])
//             .allow_methods(vec![
//                 Method::GET,
//                 Method::POST,
//             ])
// }

pub fn get_cors_layer() -> CorsLayer {

    let allowed_origins = [
        "http://localhost:3000".parse().unwrap(),
        "http://localhost:8001".parse().unwrap()
    ];

    return
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(allowed_origins))
            .allow_headers(vec![
                header::CONTENT_TYPE,
            ])
            .allow_methods(vec![
                Method::GET,
                Method::POST,
            ])
}