use serde::Serialize;
use serde_json::Value;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use tap::Pipe;

#[derive(Debug, Serialize)]
struct Response {
    msg: String
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(_: LambdaEvent<Value>) -> Result<Response, Error> {
    // Extract some useful information from the request

    Response {
        msg: "Hello, world!".into()
    }
    .pipe(Ok)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
