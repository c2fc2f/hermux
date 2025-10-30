use crate::openrouter::{BASE_URL, response};
use actix_web::{HttpResponseBuilder, Responder, get, http::StatusCode, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct PathParams {
    author: String,
    slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryParam {
    provider: Option<String>,
}

pub(super) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(parameters);
}

#[get("/api/v1/parameters/{author}/{slug}")]
async fn parameters(
    params: web::Path<PathParams>,
    query: web::Query<QueryParam>,
) -> impl Responder {
    let client: reqwest::Client = reqwest::Client::new();

    let (status, body): (
        StatusCode,
        response::Response<response::ModelParameter>,
    ) = async {
        let result: reqwest::Response = client
            .get(format!(
                "{BASE_URL}parameters/{}/{}",
                params.author, params.slug
            ))
            .query(&query.into_inner())
            .send()
            .await?;
        let status: StatusCode = result.status().as_u16().try_into()?;
        let body: response::Response<response::ModelParameter> = {
            #[cfg(debug_assertions)]
            {
                serde_path_to_error::deserialize::<_, response::ModelParameter>(
                    &mut serde_json::Deserializer::from_str(
                        &result.text().await?,
                    ),
                )?
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
