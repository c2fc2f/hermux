use actix_web::web;

mod zdr;

pub(super) fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(zdr::config);
}
