use actix_web::web;

mod chat;
mod endpoints;
mod generation;
mod models;
mod parameters;
mod providers;

pub(super) fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(chat::config)
        .configure(endpoints::config)
        .configure(generation::config)
        .configure(models::config)
        .configure(parameters::config)
        .configure(providers::config);
}
