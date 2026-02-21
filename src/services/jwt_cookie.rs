// Extract everything directly from tower_cookies to avoid dependency hell
use tower_cookies::cookie::{Cookie, SameSite, time::Duration as TimeDuration};

pub fn save_to_cookie(token: String) -> Cookie<'static> {
    let is_production =
        std::env::var("PRODUCTION").unwrap_or_else(|_| "false".to_string()) == "true";

    Cookie::build(("accessToken", token))
        .http_only(true)
        .secure(is_production)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(TimeDuration::days(1))
        .build()
}
