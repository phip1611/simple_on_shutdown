use actix_web::{get, HttpServer, App, HttpResponse, Responder};
use simple_on_shutdown::on_shutdown;

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Important that this value lives through the whole
    // lifetime of main(). This gets dropped in the end.
    on_shutdown!({
                // the actual code
                println!("This gets executed during shutdown. Don't to expensive operations here.");
    });

    HttpServer::new(|| {
        App::new()
            .service(get_index)
    })
        .bind(format!("localhost:{}", 8080))?
        .run()
        .await
}
