#[macro_use]
extern crate log;

mod data;
mod error;

use crate::error::ServerError;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use chrono::Duration;
use config::{Config, ConfigError, Environment};
use data::Database;
use rand::RngCore;
use serde::Deserialize;
use entities::{LoginChallenge, Operation, Account};

#[derive(Deserialize)]
struct ServerConfig {
    bind_addr: String,
    db_addr: String,
    migration_addr: String,
}

impl ServerConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        cfg.merge(Environment::new())?;
        cfg.try_into()
    }
}

async fn login(
    id: Identity,
    db: web::Data<Database>,
    challenge: web::Json<LoginChallenge>,
) -> Result<HttpResponse, AWError> {
    let secret = web::block(move || -> Result<String, ServerError> {
        let secret = db.get_secret()?;
        Ok(secret)
    })
    .await?;

    if secret == challenge.token {
        id.remember("admin".to_owned());
        Ok(HttpResponse::Created().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

async fn operation(
    id: Identity,
    db: web::Data<Database>,
    item: web::Json<Operation>,
) -> Result<HttpResponse, AWError> {
    id.identity().ok_or(ServerError::UnauthorizedError)?;
    web::block(move || -> Result<(), ServerError> {
        db.insert_operation(&item)?;
        db.update_balance(&item.from, -item.amount)?;
        db.update_balance(&item.to, item.amount)?;

        Ok(())
    })
    .await?;

    Ok(HttpResponse::Created().finish())
}

async fn add_account(
    id: Identity,
    db: web::Data<Database>,
    account: web::Json<Account>,
) -> Result<HttpResponse, AWError> {
    id.identity().ok_or(ServerError::UnauthorizedError)?;
    web::block(move || -> Result<(), ServerError> {
        db.add_account(&account)?;

        Ok(())
    })
    .await?;

    Ok(HttpResponse::Created().finish())
}

async fn account(id: Identity, db: web::Data<Database>) -> Result<HttpResponse, AWError> {
    id.identity().ok_or(ServerError::UnauthorizedError)?;
    let result = web::block(move || db.get_accounts()).await?;

    Ok(HttpResponse::Ok().json(result))
}

#[actix_rt::main]
async fn main() -> Result<(), ServerError> {
    dotenv::dotenv()?;
    env_logger::init();

    let cfg = ServerConfig::new()?;
    info!("Configuration is load successfully.");

    let db = Database::new(cfg.db_addr);
    db.migrate(cfg.migration_addr)?;

    let mut key = [0u8; 128];
    rand::thread_rng().fill_bytes(&mut key);

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&key)
                    .name("token")
                    .max_age(Duration::days(365).num_seconds())
                    .secure(false),
            ))
            .data(db.clone())
            .data(web::JsonConfig::default().limit(4096))
            .wrap(middleware::Logger::default())
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/operation").route(web::post().to(operation)))
            .service(web::resource("/account").route(web::get().to(account)).route(web::post().to(add_account)))
            .service(actix_files::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(cfg.bind_addr)?
    .run()
    .await?;

    Ok(())
}
