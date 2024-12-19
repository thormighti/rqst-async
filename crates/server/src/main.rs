use miniserve::{Content, Request, Response};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Chat {
    messages: Vec<String>,
}

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(req: Request) -> Response {
    match req {
        Request::Post(body) => {
            let chat: Chat = serde_json::from_str(&body)
                .map_err(|_| miniserve::http::StatusCode::BAD_REQUEST)?;

            let mut messages = chat.messages;
            messages.push("And how does that make you feel?".to_string());

            let response = Chat { messages };
            let json = serde_json::to_string(&response)
                .map_err(|_| miniserve::http::StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Content::Json(json))
        }
        _ => Err(miniserve::http::StatusCode::METHOD_NOT_ALLOWED),
    }
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}
