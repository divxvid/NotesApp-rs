use axum::http::Method;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ])
        .allow_origin(AllowOrigin::mirror_request())
        .allow_headers(AllowHeaders::mirror_request())
        .allow_credentials(true)
}
