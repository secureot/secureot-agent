use axum::{Router, Json};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::metrics::Metrics;
use prometheus::TextEncoder;

#[derive(Serialize, Deserialize)]
struct BpfFilter {
    filter: String,
}

pub async fn start_server(state: Arc<Metrics>, filters: Arc<Mutex<Vec<String>>>) {
    let app = Router::new()
        .route("/stats", get(move || get_stats(Arc::clone(&state))))
        .route("/set_bpf", post(move |payload| set_bpf_filter(payload, Arc::clone(&filters))))
        .route("/metrics", get(move || get_metrics()));

    println!("🔗 API REST activa en puerto 8080...");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_stats(state: Arc<Metrics>) -> Json<String> {
    Json(format!(
        r#"{{
            "packets_processed": {},
            "cpu_usage": {},
            "memory_usage": {}
        }}"#,
        state.packets_processed.get(),
        state.cpu_usage.get(),
        state.memory_usage.get(),
    ))
}

async fn set_bpf_filter(Json(payload): Json<BpfFilter>, filters: Arc<Mutex<Vec<String>>>) {
    let mut active_filters = filters.lock().unwrap();
    
    // Validar filtro antes de agregarlo
    if payload.filter.is_empty() || active_filters.contains(&payload.filter) {
        println!("⚠️ Filtro BPF inválido o duplicado: {}", payload.filter);
        return;
    }

    active_filters.push(payload.filter.clone());
    println!("✅ Filtro BPF aplicado: {}", payload.filter);
}

async fn get_metrics() -> Json<String> {
    let encoder = TextEncoder::new();
    let mut buffer = String::new();
    encoder.encode_utf8(&prometheus::gather(), &mut buffer).unwrap();
    Json(buffer) // Retornar métricas como JSON
}
