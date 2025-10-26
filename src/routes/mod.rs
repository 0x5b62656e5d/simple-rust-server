use actix_governor::GovernorConfigBuilder;
use actix_web::web;

pub mod coffee;
pub mod echo;
pub mod hello;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let governor = GovernorConfigBuilder::default()
        .requests_per_minute(10)
        .burst_size(1)
        .finish()
        .expect("Failed to create governor config");

    cfg.service(
        web::scope("/api")
            .wrap(actix_governor::Governor::new(&governor))
            .service(hello::hello)
            .service(web::scope("/echo").service(echo::echo))
            .service(web::scope("/coffee").service(coffee::coffee)),
    );
}
