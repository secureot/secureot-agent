use axum::{Router, Json};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::metrics::AppState;

#[derive(Serialize, Deserialize)]
struct BpfFilter {
    filter: String,
}

pub async fn start_server(state: Arc<AppState>, filters: Arc<Mutex<Vec<String>>>) {
    let app = Router::new()
        .route("/stats", get(move || get_stats(Arc::clone(&state))))
        .route("/set_bpf", post(move |payload| set_bpf_filter(payload, Arc::clone(&filters))));

    println!("🔗 API REST activa en puerto 8080...");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_stats(state: Arc<AppState>) -> Json<String> {
    let stats = format!(
        r#"{{
            "packets_captured": {},
            "packets_filtered": {}
        }}"#,
        state.packets_captured.get(),
        state.packets_filtered.get(),
    );
    Json(stats)
}

async fn set_bpf_filter(Json(payload): Json<BpfFilter>, filters: Arc<Mutex<Vec<String>>>) {
    let mut active_filters = filters.lock().unwrap();
    active_filters.push(payload.filter.clone());
    println!("✅ Filtro BPF aplicado: {}", payload.filter);
}
