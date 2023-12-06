use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use quick_js::{Context, JsValue};
use std::fs;

// Function to execute JavaScript code from a file
// Note: `const` cannot be used with `String::to_string()`; use `&'static str` instead
const FILE_PATH: &str = "./src/action.js";

fn execute_js_from_file() -> Result<JsValue, Box<dyn std::error::Error>> {
    let js_code = fs::read_to_string(FILE_PATH)?;
    let context = Context::new()?;
    context.eval(&js_code)?;
    // Since the function takes no arguments, we can pass an empty Vec<JsValue>
    let args: Vec<JsValue> = Vec::new();
    context.call_function("action", args)
           .map_err(|e| e.into()) // Converts ExecutionError into Box<dyn std::error::Error>
}


// Handler for POST request to "/"
async fn execute_js_file_endpoint() -> impl Responder {
    match execute_js_from_file() {
        Ok(result) => HttpResponse::Ok().body(format!("Result: {:?}", result)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(execute_js_file_endpoint)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
