use axum::response::IntoResponse;
use loco_rs::controller::{format, views::ViewRenderer};
use loco_rs::Result;
use serde_json::json;

pub fn home(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "home/index.html", json!({}))
}
