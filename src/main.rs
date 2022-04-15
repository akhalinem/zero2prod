use std::net::TcpListener;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");

    zero2prod::startup::run(listener)?.await
}
