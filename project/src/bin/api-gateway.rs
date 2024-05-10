use lambda_http::{run, service_fn, tracing, Error, IntoResponse, Request, RequestPayloadExt};
use serde::{Deserialize, Serialize};
use serde_json as json;
use tap::Pipe;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    id: String,
}

#[derive(Debug, Serialize)]
struct Output {
    msg: String,
}

async fn function_handler(req: Request) -> Result<impl IntoResponse, Error> {
    let input: Input = req.payload()?.expect("Failed to get payload");

    json::to_value(Output {
        msg: format!("hello {0}!", input.id),
    })?
    .pipe(Ok)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
