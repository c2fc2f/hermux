use actix_web::web;

mod v1;

pub(super) fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(v1::config);
}
