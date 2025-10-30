//! Sub-Module for regrouping response type of openrouter API

use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

pub(crate) type Unknown = serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ModerationErrorMetadata {
    pub(crate) reasons: Vec<String>,
    pub(crate) flagged_input: String,
    pub(crate) provider_name: String,
    pub(crate) model_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub(crate) struct ErrorResponseInner {
    pub(crate) code: usize,
    pub(crate) message: String,
    pub(crate) metadata: Option<ErrorMetadata>,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub(crate) struct ListAvailableModels {
    pub(crate) data: Vec<ModelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ModelInfo {
    pub(crate) id: String,
    pub(crate) canonical_slug: String,
    pub(crate) name: String,
    pub(crate) created: u64,
    pub(crate) pricing: PricingInfo,
    pub(crate) context_length: Option<u64>,
    pub(crate) architecture: ArchitectureInfo,
    pub(crate) top_provider: TopProviderInfo,
    pub(crate) per_request_limits: Option<PerRequestLimitsInfo>,
    pub(crate) supported_parameters: Vec<String>,
    pub(crate) default_parameters: Option<DefaultParametersInfo>,
    pub(crate) hugging_face_id: Option<String>,
    pub(crate) description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PricingInfo {
    pub(crate) prompt: String,
    pub(crate) completion: String,
    pub(crate) request: Option<String>,
    pub(crate) image: Option<String>,
    pub(crate) image_output: Option<String>,
    pub(crate) audio: Option<String>,
    pub(crate) input_audio_cache: Option<String>,
    pub(crate) web_search: Option<String>,
    pub(crate) internal_reasoning: Option<String>,
    pub(crate) input_cache_read: Option<String>,
    pub(crate) input_cache_write: Option<String>,
    pub(crate) discount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArchitectureInfo {
    pub(crate) modality: Option<String>,
    pub(crate) input_modalities: Vec<String>,
    pub(crate) output_modalities: Vec<String>,
    pub(crate) tokenizer: String,
    pub(crate) instruct_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TopProviderInfo {
    pub(crate) is_moderated: bool,
    pub(crate) context_length: Option<u64>,
    pub(crate) max_completion_tokens: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PerRequestLimitsInfo {
    pub(crate) prompt_tokens: Option<u64>,
    pub(crate) completion_tokens: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DefaultParametersInfo {
    pub(crate) temperature: Option<f64>,
    pub(crate) top_p: Option<f64>,
    pub(crate) frequency_penalty: Option<f64>,
}

impl SuccessResponse for ListAvailableModels {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TotalCountAvailableModels {
    pub(crate) data: Count,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Count {
    pub(crate) count: u32,
}

impl SuccessResponse for TotalCountAvailableModels {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ListModelsFilteredUserPreferences {
    pub(crate) data: Vec<ModelInfo>,
}

impl SuccessResponse for ListModelsFilteredUserPreferences {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ListEndpointsModel {
    pub(crate) data: ListEndpointsModelInner,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ListEndpointsModelInner {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) created: u64,
    pub(crate) description: String,
    pub(crate) architecture: ArchitectureInfo,
    pub(crate) endpoints: Vec<EndpointInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EndpointInfo {
    pub(crate) name: String,
    pub(crate) model_name: String,
    pub(crate) context_length: u64,
    pub(crate) pricing: PricingInfo,
    pub(crate) provider_name: String,
    pub(crate) tag: String,
    pub(crate) quantization: String,
    pub(crate) max_completion_tokens: Option<u64>,
    pub(crate) max_prompt_tokens: Option<u64>,
    pub(crate) supported_parameters: Vec<String>,
    pub(crate) uptime_last_30m: Option<f64>,
    pub(crate) supports_implicit_caching: bool,
    pub(crate) status: i64,
}

impl SuccessResponse for ListEndpointsModel {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ListEndpointsZDR {
    pub(crate) data: Vec<EndpointInfo>,
}

impl SuccessResponse for ListEndpointsZDR {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ModelParameter {
    pub(crate) data: ModelParameterInner,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ModelParameterInner {
    pub(crate) model: String,
    pub(crate) supported_parameters: Vec<String>,
}

impl SuccessResponse for ModelParameter {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ListProviders {
    pub(crate) data: Vec<ProviderInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProviderInfo {
    pub(crate) name: String,
    pub(crate) slug: String,
    pub(crate) privacy_policy_url: Option<String>,
    pub(crate) terms_of_service_url: Option<String>,
    pub(crate) status_page_url: Option<String>,
}

impl SuccessResponse for ListProviders {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GenerationInfo {
    pub(crate) data: GenerationInfoInner,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GenerationInfoInner {
    pub(crate) id: String,
    pub(crate) upstream_id: Option<String>,
    pub(crate) total_cost: f64,
    pub(crate) cache_discount: Option<f64>,
    pub(crate) upstream_inference_cost: Option<f64>,
    pub(crate) created_at: String,
    pub(crate) model: String,
    pub(crate) app_id: Option<u64>,
    pub(crate) streamed: Option<bool>,
    pub(crate) cancelled: Option<bool>,
    pub(crate) provider_name: Option<String>,
    pub(crate) latency: Option<u64>,
    pub(crate) moderation_latency: Option<u64>,
    pub(crate) generation_time: Option<u64>,
    pub(crate) finish_reason: Option<String>,
    pub(crate) tokens_prompt: Option<u64>,
    pub(crate) tokens_completion: Option<u64>,
    pub(crate) native_tokens_prompt: Option<u64>,
    pub(crate) native_tokens_completion: Option<u64>,
    pub(crate) native_tokens_completion_images: Option<u64>,
    pub(crate) native_tokens_reasoning: Option<u64>,
    pub(crate) native_tokens_cached: Option<u64>,
    pub(crate) num_media_prompt: Option<u64>,
    pub(crate) num_input_audio_prompt: Option<u64>,
    pub(crate) num_media_completion: Option<u64>,
    pub(crate) num_search_results: Option<u64>,
    pub(crate) origin: String,
    pub(crate) usage: f64,
    pub(crate) is_byok: bool,
    pub(crate) native_finish_reason: Option<String>,
    pub(crate) external_user: Option<String>,
    pub(crate) api_type: Option<String>,
}

impl SuccessResponse for GenerationInfo {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum Response<T>
where
    T: SuccessResponse,
{
    Error(ErrorResponse),
    Success(T),
}

impl<T> From<T> for Response<T>
where
    T: SuccessResponse,
{
    fn from(value: T) -> Self {
        Self::Success(value)
    }
}

impl<T> From<ErrorResponse> for Response<T>
where
    T: SuccessResponse,
{
    fn from(value: ErrorResponse) -> Self {
        Self::Error(value)
    }
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
