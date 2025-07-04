//! Sub-Module for regrouping response type of openrouter API

use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

pub(crate) type Unknown = serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ModerationErrorMetadata {
    pub(crate) reasons: Vec<String>,
    pub(crate) flagged_input: String,
    pub(crate) provider_name: String,
    pub(crate) model_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProviderErrorMetadata {
    pub(crate) provider_name: String,
    pub(crate) raw: Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ErrorMetadata {
    Moderation(ModerationErrorMetadata),
    Provider(ProviderErrorMetadata),
    Other(HashMap<String, Unknown>),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ErrorResponseInner {
    pub(crate) code: usize,
    pub(crate) message: String,
    pub(crate) metadata: Option<ErrorMetadata>,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub(crate) struct ErrorResponse {
    pub(crate) error: ErrorResponseInner,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

pub(crate) trait SuccessResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ListAvailableModels {
    pub(crate) data: Vec<ModelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ModelInfo {
    pub(crate) id: String,
    pub(crate) name: String,
    // Fuck off this field doesn't exist in their API
    pub(crate) canonical_slug: Option<String>,
    // Returns only an unsigned integer
    pub(crate) created: u64,
    pub(crate) description: String,
    pub(crate) architecture: ArchitectureInfo,
    pub(crate) top_provider: TopProviderInfo,
    pub(crate) pricing: PricingInfo,
    // Returns only an unsigned integer
    pub(crate) context_length: Option<u64>,
    pub(crate) hugging_face_id: Option<String>,
    pub(crate) per_request_limits: Option<HashMap<String, Unknown>>,
    pub(crate) supported_parameters: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ArchitectureInfo {
    // Fuck off this field doesn't exist in their API
    pub(crate) modality: Option<String>,
    pub(crate) input_modalities: Vec<String>,
    pub(crate) output_modalities: Vec<String>,
    pub(crate) tokenizer: String,
    pub(crate) instruct_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TopProviderInfo {
    pub(crate) is_moderated: bool,
    // Returns only an unsigned integer
    pub(crate) context_length: Option<u64>,
    // Returns only an unsigned integer
    pub(crate) max_completion_tokens: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PricingInfo {
    pub(crate) prompt: String,
    pub(crate) completion: String,
    // Fuck off this field is not an option in their API
    pub(crate) image: Option<String>,
    // Fuck off this field is not an option in their API
    pub(crate) request: Option<String>,
    // Fuck off this field is not an option in their API
    pub(crate) input_cache_read: Option<String>,
    // Fuck off this field is not an option in their API
    pub(crate) input_cache_write: Option<String>,
    // Fuck off this field is not an option in their API
    pub(crate) web_search: Option<String>,
    // Fuck off this field is not an option in their API
    pub(crate) internal_reasoning: Option<String>,
}

impl SuccessResponse for ListAvailableModels {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Response<T>
where
    T: SuccessResponse,
{
    Error(ErrorResponse),
    Success(T),
}

pub(crate) type Result<T> = std::result::Result<T, ErrorResponse>;

impl<T> From<Result<T>> for Response<T>
where
    T: SuccessResponse,
{
    fn from(value: Result<T>) -> Self {
        match value {
            Err(err) => Self::Error(err),
            Ok(succ) => Self::Success(succ),
        }
    }
}

impl<T> From<Response<T>> for Result<T>
where
    T: SuccessResponse,
{
    fn from(value: Response<T>) -> Self {
        match value {
            Response::Error(err) => Self::Err(err),
            Response::Success(succ) => Self::Ok(succ),
        }
    }
}
