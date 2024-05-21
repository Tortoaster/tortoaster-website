use axum::response::Redirect;
use axum_extra::{extract::WithRejection, routing::TypedPath};
use axum_oidc::OidcRpInitiatedLogout;

use crate::{api::projects::ListProjectsUrl, config::AppConfig, error::WithPageRejection};

#[derive(Copy, Clone, Debug, Default, TypedPath)]
#[typed_path("/login")]
pub struct LoginUrl;

pub async fn login(_: LoginUrl) -> Redirect {
    Redirect::temporary(&ListProjectsUrl.to_string())
}

#[derive(Copy, Clone, Debug, Default, TypedPath)]
#[typed_path("/logout")]
pub struct LogoutUrl;

pub async fn logout(
    _: LogoutUrl,
    WithRejection(logout, _): WithPageRejection<OidcRpInitiatedLogout>,
) -> OidcRpInitiatedLogout {
    logout.with_post_logout_redirect(AppConfig::get().oidc.redirect_url.parse().unwrap())
}
