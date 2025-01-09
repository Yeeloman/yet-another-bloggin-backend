mod routes;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use std::fmt::Write;

const ADDRS: &str = "127.0.0.1:8000";


#[actix_web::main]
async fn main() -> std::io::Result<()> {


    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();
    env_logger::init();

    println!("Server started at http://{}", ADDRS);

    HttpServer::new(move || {
        let logger = Logger::new(&generate_dynamic_log_format().as_str());
        App::new()
            .wrap(logger)
            .wrap(actix_web::middleware::NormalizePath::trim())
            .configure(routes::init)
    })
    .bind(ADDRS)?
    .run()
    .await

}

fn generate_dynamic_log_format() -> String {
    let log_message = "# %s - %r Served in %T #";
    let message_length = log_message.len();
    let border_length = (message_length) * 2; // Adding space for border padding
    let border = "#".repeat(border_length);

    // Create the formatted log
    let mut formatted_log = String::new();
    writeln!(formatted_log, "\n{}", border).unwrap();
    writeln!(formatted_log, "{}", log_message).unwrap();
    writeln!(formatted_log, "{}", border).unwrap();

    formatted_log
}
