mod count;
mod endpoints;
mod user;

use crate::openrouter::{BASE_URL, response};
use actix_web::{HttpResponseBuilder, Responder, get, http::StatusCode, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct QueryParam {
    category: Option<String>,
    supported_parameters: Option<String>,
    use_rss: Option<String>,
    use_rss_chat_links: Option<String>,
}

pub(super) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(models)
        .configure(count::config)
        .configure(user::config)
        .configure(endpoints::config);
}

#[get("/api/v1/models")]
async fn models(query: web::Query<QueryParam>) -> impl Responder {
    let client: reqwest::Client = reqwest::Client::new();

    let (status, body): (
        StatusCode,
        response::Response<response::ListAvailableModels>,
    ) = async {
        let result: reqwest::Response = client
            .get(format!("{BASE_URL}models"))
            .query(&query.into_inner())
            .send()
            .await?;
        let status: StatusCode = result.status().as_u16().try_into()?;
        let body: response::Response<response::ListAvailableModels> = {
            #[cfg(debug_assertions)]
            {
                serde_path_to_error::deserialize::<
                    _,
                    response::ListAvailableModels,
                >(&mut serde_json::Deserializer::from_str(
                    &result.text().await?,
                ))?
                .into()
            }
            #[cfg(not(debug_assertions))]
            result.json().await?
        };
        anyhow::Ok((status, body))
    }
    .await
    .unwrap_or_else(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            response::Response::Error(response::ErrorResponse {
                error: response::ErrorResponseInner {
                    code: 500,
                    message: format!("{err:#?}"),
                    metadata: None,
                },
            }),
        )
    });
    HttpResponseBuilder::new(status).json(body)
}
