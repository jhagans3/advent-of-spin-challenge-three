use http::StatusCode;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Params, Request, Router};
use spin_sdk::http_component;
use spin_sdk::llm::{infer, InferencingModel::Llama2Chat};

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

    let prompt = format!("Create a poem in {place} with characters {c} with things {o}");
    let inference_res = infer(Llama2Chat, &prompt)?;

    let response_body = serde_json::to_string(&StoryResponse {
        story: inference_res.text,
    })?;

    Ok(http::Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(response_body)?)
}
