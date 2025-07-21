use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(ToSchema)]
struct AppInfo {
    app_name: String,
    version: String,
}

struct AppState {
    app_name: String,
    version: String,
    visit_count: Mutex<u32> // compteur protégé par Mutex, thread-safe 
}

#[utoipa::path(
    get,
    path = "/",
    responses((status = 200, description = "Accueil")),
    tag = "API Calculator"
)]
#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut count = data.visit_count.lock().unwrap();
    *count += 1;
    let message = format!(
        "Bienvenue sur {} (v{})\nNombre de visites: {}",
        data.app_name,
        data.version,
        count
    );
    HttpResponse::Ok().body(message)
}

#[utoipa::path(
    get,
    path = "/status",
    responses((status = 200, description = "Status")),
    tag = "API Calculator"
)]
#[get("/status")]
async fn status(data: web::Data<AppState>) -> impl Responder {
    let message = format!(
        "Application: {}\nVersion: {}",
        data.app_name,
        data.version
    );
    HttpResponse::Ok().body(message)
}

#[derive(OpenApi)]
#[openapi(
    paths(index, status),
    components(schemas(AppInfo)),
    tags((name = "API Calculator", description = "API pour calculatrice"))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr: &str = "0.0.0.0";
    let port: u16 = 8080;

    let app_state = web::Data::new(AppState {
        app_name: "API Calculator".to_string(),
        version: "1.0.0".to_string(),
        visit_count: Mutex::new(0)
    });

    // Lancement du server
    HttpServer::new(move || {
        App::new()
            .service(SwaggerUi::new("/doc/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi()))
            .app_data(app_state.clone())
            .service(index)
            .service(status)
    })
    .bind((addr, port))?
    .run()
    .await
}