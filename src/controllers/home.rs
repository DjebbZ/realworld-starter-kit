#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::views;

#[debug_handler]
pub async fn echo(req_body: String) -> String {
    req_body
}

#[debug_handler]
pub async fn home(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    views::home::home(v)
}

#[debug_handler]
async fn login_page(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    views::auth::login_page(v)
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(home))
        .add("/login", get(login_page))
}
