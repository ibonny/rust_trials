mod user_service;

use spring::{App, auto_config};
use spring_web::extractor::Component;
use spring_web::{WebConfigurator, WebPlugin, axum::response::IntoResponse, extractor::Path};
use spring_web::{get, route};
use user_service::UserService;

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    App::new().add_plugin(WebPlugin).run().await
}

#[get("/")]
async fn hello_world() -> impl IntoResponse {
    "hello world"
}

#[route("/hello/{name}", method = "GET", method = "POST")]
async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("hello {name}")
}

#[route("/testout", method="GET")]
async fn testout(Component(user_service): Component<UserService>) -> impl IntoResponse {
    format!("{}", user_service.stuff_to_do().await)
}
