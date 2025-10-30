//! Openrouter API handler

use actix_web::web;

const BASE_URL: &str = "https://openrouter.ai/api/v1/";

mod api;
mod response;

pub(super) fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(api::config);
}
