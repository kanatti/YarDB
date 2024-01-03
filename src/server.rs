use std::io::{self, Write};

use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use tokio::net::TcpListener;

const LOGO: &str = r#"
 __  __     ______     ______     _____     ______   
/\ \_\ \   /\  __ \   /\  == \   /\  __-.  /\  == \  
\ \____ \  \ \  __ \  \ \  __<   \ \ \/\ \ \ \  __<  
 \/\_____\  \ \_\ \_\  \ \_\ \_\  \ \____-  \ \_____\
  \/_____/   \/_/\/_/   \/_/ /_/   \/____/   \/_____/
"#;

#[tokio::main]
async fn main() {
    print_and_flush(LOGO);
    print_and_flush("\nYarDB Version 0.0.1\n\n");

    let app = Router::new().route("/_health", get(health));

    print_and_flush("Listening on port 3050\n");
    let listener = TcpListener::bind("0.0.0.0:3050").await.unwrap();

    print_and_flush("Listening to requests\n");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> (StatusCode, Json<HealthStats>) {
    (
        StatusCode::OK,
        Json(HealthStats {
            status: ClusterStatus::Green,
        }),
    )
}

#[derive(Serialize)]
struct HealthStats {
    status: ClusterStatus,
}

#[derive(Serialize)]
enum ClusterStatus {
    Green,
    Yellow,
    Red,
}

fn print_and_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}
