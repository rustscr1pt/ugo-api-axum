use axum::http::{header, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};

// For setting up cors layer in the future.
// https://github.com/tower-rs/tower-http/issues/194

// More info about setting up CORS
// https://docs.rs/tower-http/0.5.2/tower_http/cors/struct.AllowOrigin.html
// https://docs.rs/tower-http/0.5.2/tower_http/cors/struct.CorsLayer.html#method.allow_origin


// OLD VERSION - RETURN IF SMTH GOES WRONG
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


// Allowed origins
// https://ugo-vape.ru
// https://walgreenlogistics.ru

// Replace cors route here! + check if there is a "/" at the end of URL, it shouldn't be there!
// Otherwise, it won't work as expected

pub fn get_cors_layer() -> CorsLayer {

    let allowed_origins = [
        "https://ugo-vape.ru".parse().unwrap(),
        "https://www.ugo-vape.ru".parse().unwrap(),
        "https://walgreenlogistics.ru".parse().unwrap(),
        "https://www.walgreenlogistics.ru".parse().unwrap(),
        "http://localhost:3000".parse().unwrap() // Opened for testing
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