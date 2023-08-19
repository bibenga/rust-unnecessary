use actix_web::{get, middleware::Logger, post, web, http, App, HttpServer, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct User {
    username: String,
    is_active_flg: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.username, self.is_active_flg)
    }
}

impl User {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
}

#[utoipa::path(get, path = "/hello/{name}")]
#[get("/hello/{name}")]
async fn say_hello(name: web::Path<String>) -> impl Responder {
    web::Json(User {
        username: name.clone(),
        is_active_flg: false,
    })
}

#[utoipa::path(
    post, path = "/hello",
    responses(
        (status = 201, description = "User crated succesfully", body = User),
    ),
)]
#[post("/hello")]
async fn create_hello(user: web::Json<User>) -> impl Responder {
    (user, http::StatusCode::CREATED)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting...");

    #[derive(OpenApi)]
    #[openapi(
        paths(
            say_hello,
            create_hello,
        ),
        components(
            schemas(User)
        ),
        tags(
            (name = "palabras", description = "Palabras API")
        ),
        // modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(say_hello)
            .service(create_hello)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
