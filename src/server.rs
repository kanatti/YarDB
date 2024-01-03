use std::{
    io::{self, Write},
    sync::Arc,
};

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use tokio::net::TcpListener;
use yardb::table::Table;

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

    let table = Table::new("test".to_owned());

    let state = AppState {
        cluster_id: "1".to_string(),
        table,
    };

    let app = Router::new()
        .route("/_health", get(health))
        .route("/stats", get(stats))
        .with_state(Arc::new(state));

    print_and_flush("Listening on port 3050\n");
    let listener = TcpListener::bind("0.0.0.0:3050").await.unwrap();

    print_and_flush("Listening to requests\n");
    axum::serve(listener, app).await.unwrap();
}

async fn health(State(state): State<Arc<AppState>>) -> (StatusCode, Json<ServerInfo>) {
    (
        StatusCode::OK,
        Json(ServerInfo {
            status: ClusterStatus::Green,
            cluster_id: state.cluster_id.clone(),
        }),
    )
}

async fn stats(State(state): State<Arc<AppState>>) -> (StatusCode, Json<StatsResponse>) {
    let table_stats = state.table.stats();
    (
        StatusCode::OK,
        Json(StatsResponse {
            num_rows: table_stats.num_rows,
            num_pages: table_stats.num_pages,
        }),
    )
}

#[derive(Serialize)]
struct StatsResponse {
    num_rows: usize,
    num_pages: usize,
}

#[derive(Serialize)]
struct ServerInfo {
    status: ClusterStatus,
    cluster_id: String,
}

#[derive(Serialize)]
enum ClusterStatus {
    Green,
    Yellow,
    Red,
}

struct AppState {
    cluster_id: String,
    table: Table,
}

fn print_and_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}
