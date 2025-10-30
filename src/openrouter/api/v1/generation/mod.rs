use crate::openrouter::{BASE_URL, response};
use actix_web::{HttpResponseBuilder, Responder, get, http::StatusCode, web};
use serde::{Deserialize, Serialize};

pub(super) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(generation);
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryParam {
    #[serde(deserialize_with = "validate_id")]
    id: String,
}

fn validate_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let id = String::deserialize(deserializer)?;
    if id.is_empty() {
        return Err(serde::de::Error::custom(
            "id must have at least one character",
        ));
    }
    Ok(id)
}

#[get("/api/v1/generation")]
async fn generation(query: web::Query<QueryParam>) -> impl Responder {
    let client: reqwest::Client = reqwest::Client::new();

    let (status, body): (
        StatusCode,
        response::Response<response::GenerationInfo>,
    ) = async {
        let result: reqwest::Response = client
            .get(format!("{BASE_URL}generation",))
            .query(&query.into_inner())
            .send()
            .await?;
        let status: StatusCode = result.status().as_u16().try_into()?;
        let body: response::Response<response::GenerationInfo> = {
            #[cfg(debug_assertions)]
            {
                serde_path_to_error::deserialize::<_, response::GenerationInfo>(
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
