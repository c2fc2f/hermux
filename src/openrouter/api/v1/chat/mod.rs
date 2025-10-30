use crate::openrouter::{
    BASE_URL,
    response::{self, Unknown},
};
use actix_web::{HttpResponseBuilder, Responder, http::StatusCode, post, web};

pub(super) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(generation);
}

#[post("/api/v1/generation")]
async fn generation(body: web::Json<Unknown>) -> impl Responder {
    let client: reqwest::Client = reqwest::Client::new();

    let (status, body): (
        StatusCode,
        response::Response<response::GenerationInfo>,
    ) = async {
        let result: reqwest::Response = client
            .post(format!("{BASE_URL}generation",))
            .json(&body)
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
