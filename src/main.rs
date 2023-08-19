use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use log::info;
use std::fmt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug)]
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
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting...");

    #[derive(OpenApi)]
    #[openapi(
        paths(
            say_hello,
        ),
        components(
            // schemas(todo::Todo, todo::TodoUpdateRequest, todo::ErrorResponse)
        ),
        tags(
            (name = "palabras", description = "Palabras API")
        ),
        // modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    HttpServer::new(|| {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(say_hello)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
