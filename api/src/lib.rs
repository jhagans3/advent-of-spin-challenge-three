use http::StatusCode;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Params, Request, Router};
use spin_sdk::http_component;
use spin_sdk::llm::{infer_with_options, InferencingModel::Llama2Chat, InferencingParams};

#[derive(Debug, Deserialize, Clone)]
struct PromptRequest {
    place: String,
    characters: Vec<String>,
    objects: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct StoryResponse {
    story: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::new();

    router.post("/", post_handler);

    Ok(router.handle(req))
}

fn post_handler(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let json = serde_json::from_slice::<PromptRequest>(req.body())?;
    let PromptRequest {
        place,
        characters,
        objects,
    } = json;
    let c = characters.join(",");
    let o = objects.join(",");

    let prompt = format!(
        "Create a poem in {place} with characters {c} with things {o} in the style of Dr. Seuss"
    );
    let inference_res = infer_with_options(
        Llama2Chat,
        &prompt,
        InferencingParams {
            max_tokens: 500,
            repeat_penalty: 1.1,
            repeat_penalty_last_n_token_count: 64,
            temperature: 0.8,
            top_k: 40,
            top_p: 0.9,
        },
    )?;

    let response_body = serde_json::to_string(&StoryResponse {
        story: inference_res.text,
    })?;

    Ok(http::Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(response_body)?)
}
