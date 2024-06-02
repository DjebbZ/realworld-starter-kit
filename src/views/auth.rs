use axum::response::IntoResponse;
use loco_rs::controller::{format, views::ViewRenderer};
use loco_rs::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::_entities::users;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: user.email_verified_at.is_some(),
        }
    }
}

pub fn login_page(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "auth/login_page.html", json!({}))
}
