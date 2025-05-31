use axum::{Router, Json};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use prometheus::{IntCounter, Opts, Registry};
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize)]
struct BpfFilter {
    filter: String,
}

struct AppState {
    active_filters: Arc<Mutex<Vec<String>>>,
    packets_captured: IntCounter,
    packets_filtered: IntCounter,
    bytes_transmitted: IntCounter,
}

// Iniciar el servidor REST
pub async fn start_server() {
    let registry = Registry::new();
    let state = Arc::new(AppState {
        active_filters: Arc::new(Mutex::new(Vec::new())),
        packets_captured: IntCounter::new("packets_captured", "Total de paquetes capturados").unwrap(),
        packets_filtered: IntCounter::new("packets_filtered", "Total de paquetes filtrados").unwrap(),
        bytes_transmitted: IntCounter::new("bytes_transmitted", "Total de datos transmitidos").unwrap(),
    });

    registry.register(Box::new(state.packets_captured.clone())).unwrap();
    registry.register(Box::new(state.packets_filtered.clone())).unwrap();
    registry.register(Box::new(state.bytes_transmitted.clone())).unwrap();

    let app = Router::new()
        .route("/stats", get(move || get_stats(state.clone())))
        .route("/set_bpf", post(move |payload| set_bpf_filter(payload, state.clone())));

    println!("🔗 API REST activa en el puerto 8080...");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Endpoint para estadísticas de rendimiento
async fn get_stats(state: Arc<AppState>) -> Json<String> {
    let stats = format!(
        r#"{{
            "packets_captured": {},
            "packets_filtered": {},
            "bytes_transmitted": {}
        }}"#,
        state.packets_captured.get(),
        state.packets_filtered.get(),
        state.bytes_transmitted.get()
    );
    Json(stats)
}

// Endpoint para activar filtros BPF dinámicamente
async fn set_bpf_filter(Json(payload): Json<BpfFilter>, state: Arc<AppState>) {
    let mut filters = state.active_filters.lock().unwrap();
    filters.push(payload.filter.clone());
    println!("✅ Filtro BPF aplicado: {}", payload.filter);
}
// Actualizacion de codigo 2025-06-06
// Actualizacion de codigo 2025-06-05
// Actualizacion de codigo 2025-06-01
// Actualizacion de codigo 2025-05-27
// Actualizacion de codigo 2025-06-11
// Actualizacion de codigo 2025-06-09
// Actualizacion de codigo 2025-06-04
// Actualizacion de codigo 2025-06-01
// Actualizacion de codigo 2025-05-31
