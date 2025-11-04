mod args;
mod tokens;

#[cfg(feature = "auth")]
use std::{collections::HashSet, fs::read_to_string};

use std::sync::{Arc, Mutex};

use actix_web::{
    App, HttpRequest, HttpResponseBuilder, HttpServer, Responder,
    http::header,
    mime,
    web::{Bytes, Data, to},
};
use anyhow::Context;
#[cfg(feature = "auth")]
use awc::error::HeaderValue;
use awc::{Client, ClientBuilder, ClientResponse, http::StatusCode};
use clap::Parser;

use crate::tokens::{Token, TokensBalencer};

const BASE_URL: &str = "https://openrouter.ai";

struct State {
    client: Client,
    balancer: Arc<Mutex<TokensBalencer>>,
    #[cfg(feature = "auth")]
    allow: Arc<HashSet<String>>,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args: args::Args = args::Args::parse();
    let tokens: Vec<Token> = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(false)
        .quoting(true)
        .from_path(args.tokens)
        .context("Cannot open the tokens file")?
        .deserialize()
        .collect::<Result<Vec<Token>, csv::Error>>()
        .context("Invalid tokens file")?;
    let balancer: Arc<Mutex<TokensBalencer>> =
        Arc::new(Mutex::new(TokensBalencer::from(tokens)));
    #[cfg(feature = "auth")]
    let allow: Arc<HashSet<String>> = Arc::new(
        read_to_string(args.allow)
            .context("Invalid allow file")?
            .split_whitespace()
            .map(|s| s.to_string())
            .collect(),
    );
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(State {
                client: ClientBuilder::new().disable_timeout().finish(),
                balancer: Arc::clone(&balancer),
                #[cfg(feature = "auth")]
                allow: Arc::clone(&allow),
            }))
            .default_service(to(default))
    })
    .bind((args.address, args.port))?
    .run()
    .await?;

    Ok(())
}

async fn default(
    req: HttpRequest,
    body: String,
    state: Data<State>,
) -> impl Responder {
    let (status, tname, body): (StatusCode, String, Bytes) = async {
        #[cfg(feature = "auth")]
        {
            let aut: Option<&HeaderValue> =
                req.headers().get(header::AUTHORIZATION);

            let token: &str = aut
                .ok_or("Unauthorized token")?
                .to_str()
                .map_err(|e| e.to_string())?;

            let token: &str = token.strip_prefix("Bearer ").unwrap_or(token);

            if !state.allow.contains(token) {
                return Err("Unauthorized token".to_string());
            }
        }

        let token: Token = state
            .balancer
            .lock()
            .map_err(|_| "Unable to lock the tokens balancer")?
            .next()
            .ok_or("No more tokens")?;
        let mut result: ClientResponse<_> = state
            .client
            .request_from(format!("{BASE_URL}{}", req.uri()), req.head())
            .insert_header((
                header::AUTHORIZATION,
                format!("Bearer {}", token.token),
            ))
            .send_body(body)
            .await
            .map_err(|e| e.to_string())?;

        let body: Bytes = result.body().await.map_err(|e| e.to_string())?;

        Result::<_, String>::Ok((result.status(), token.name, body))
    }
    .await
    .unwrap_or_else(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "<empty>".to_string(),
            Bytes::from(format!(
                r#"{{"error":{{"code":500,"message":"{e}"}}}}"#
            )),
        )
    });

    HttpResponseBuilder::new(status)
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .insert_header(("X-TOKEN-NAME", tname))
        .body(body)
}
