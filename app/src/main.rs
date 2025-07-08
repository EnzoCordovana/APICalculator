use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
    version: String,
    visit_count: Mutex<u32> // compteur protégé par Mutex, thread-safe 
}

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

#[get("/status")]
async fn status(data: web::Data<AppState>) -> impl Responder {
    let message = format!(
        "Application: {}\nVersion: {}",
        data.app_name,
        data.version
    );

    HttpResponse::Ok().body(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = web::Data::new(AppState {
        app_name: "API Calculator".to_string(),
        version: "1.0.0".to_string(),
        visit_count: Mutex::new(0)
    });

    // Lancement du server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(status)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}