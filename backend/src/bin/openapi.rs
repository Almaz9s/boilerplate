// Binary to export OpenAPI spec to a JSON file
// Usage: cargo run --bin openapi

use backend::docs::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();
    let json = openapi.to_pretty_json().expect("Failed to serialize OpenAPI spec");
    println!("{}", json);
}
