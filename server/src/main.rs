use axum::{
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use serde::Serialize;
use std::fs;

#[derive(Debug, Clone, Serialize)]
struct CpuStats {
    timestamp: String,
    temp: f32,
    freq: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/stats", get(get_stats))
        .route("/api/current", get(get_current));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("CPU Monitor running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn get_stats() -> impl IntoResponse {
    let stats = load_data("/data");
    let stats: Vec<_> = stats.into_iter().rev().take(200).rev().collect();
    Json(stats)
}

async fn get_current() -> impl IntoResponse {
    let stats = load_data("/data");
    if let Some(last) = stats.last() {
        Json(last.clone())
    } else {
        Json(CpuStats {
            timestamp: "N/A".to_string(),
            temp: 0.0,
            freq: 0,
        })
    }
}

fn load_data(dir: &str) -> Vec<CpuStats> {
    let mut all_stats = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("csv") {
                if let Ok(content) = fs::read_to_string(&path) {
                    for line in content.lines().skip(1) {
                        if let Some(stats) = parse_line(line) {
                            all_stats.push(stats);
                        }
                    }
                }
            }
        }
    }
    all_stats.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    all_stats
}

fn parse_line(line: &str) -> Option<CpuStats> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() >= 3 {
        let temp = parts[1].parse().unwrap_or(0.0);
        let freq = parts[2].parse().unwrap_or(0);
        Some(CpuStats {
            timestamp: parts[0].to_string(),
            temp,
            freq,
        })
    } else {
        None
    }
}
