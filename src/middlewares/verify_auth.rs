use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

use crate::auth::validate_token;

pub async fn verify_auth<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let cookie_jar = CookieJar::from_headers(&request.headers());
    let jwt_token = cookie_jar
        .get("access_token")
        .map(|c| c.value().to_owned())
        .unwrap_or("nope".to_owned());

    let claims = validate_token(&jwt_token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
