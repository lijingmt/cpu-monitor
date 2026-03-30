use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Serialize)]
struct CpuStats {
    timestamp: String,
    temp: f32,
    freq: u32,
}

#[derive(Deserialize)]
struct StatsQuery {
    days: Option<String>,
}

#[derive(Clone)]
struct AppState {
    cache: HashMap<String, Vec<CpuStats>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        cache: HashMap::new(),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/api/stats", get(get_stats))
        .route("/api/current", get(get_current))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("CPU Monitor running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn get_stats(Query(params): Query<StatsQuery>, State(state): State<AppState>) -> impl IntoResponse {
    let days_key = params.days.clone().unwrap_or_else(|| "1".to_string());
    let days: u64 = days_key.parse().unwrap_or(1);

    let stats = filter_by_days(load_data("/data"), days);
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

fn filter_by_days(mut stats: Vec<CpuStats>, days: u64) -> Vec<CpuStats> {
    if stats.is_empty() {
        return stats;
    }

    // 获取最新数据的时间戳
    if let Some(last) = stats.last() {
        if let Ok(last_time) = parse_timestamp(&last.timestamp) {
            let cutoff = last_time - (days * 24 * 3600);
            stats.retain(|s| {
                if let Ok(t) = parse_timestamp(&s.timestamp) {
                    t >= cutoff
                } else {
                    false
                }
            });
        }
    }

    // 限制最多返回5000条
    if stats.len() > 5000 {
        let start = stats.len() - 5000;
        stats = stats.into_iter().skip(start).collect();
    }

    stats
}

fn parse_timestamp(ts: &str) -> Result<u64, String> {
    // 解析格式: "2026-03-30 07:05:35"
    let parts: Vec<&str> = ts.split(' ').collect();
    if parts.len() < 2 {
        return Err("Invalid format".to_string());
    }

    let date_parts: Vec<u32> = parts[0]
        .split('-')
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    let time_parts: Vec<u32> = parts[1]
        .split(':')
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    if date_parts.len() != 3 || time_parts.len() != 3 {
        return Err("Invalid format".to_string());
    }

    // 简化为秒数（不考虑闰年等，仅用于比较）
    let days_since_epoch = date_parts[0] * 365 + date_parts[1] * 30 + date_parts[2];
    let seconds = days_since_epoch * 86400 + time_parts[0] * 3600 + time_parts[1] * 60 + time_parts[2];

    Ok(seconds as u64)
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
