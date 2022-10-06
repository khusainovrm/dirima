use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fmt::format;
use std::sync::Mutex;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Buongiorno Rinat!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

async fn counter_route(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

// This struct represents state
struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(counter.clone()) // <- register the created data
            .service(web::scope("/app").route("/", web::get().to(index)))
            .service(web::scope("/counter").route("/", web::get().to(counter_route)))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
