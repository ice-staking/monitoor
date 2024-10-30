#![allow(dead_code)]
use serde::Deserialize;
use axum::{response::IntoResponse, Json}
// Represents CPU data from Telegraf
#[derive(Deserialize, Debug)]
struct CpuMetric {
    cpu: String,
    usage_user: f64,
    usage_system: f64,
    usage_idle: f64,
    usage_iowait: f64,
    usage_nice: f64,
    usage_irq: f64,
    usage_softirq: f64,
    usage_steal: f64,
}

// Represents Disk data from Telegraf
#[derive(Deserialize, Debug)]
struct DiskMetric {
    path: String,
    fstype: String,
    total: u64,
    free: u64,
    used: u64,
    used_percent: f64,
    inodes_total: u64,
    inodes_free: u64,
    inodes_used: u64,
}

// Represents Disk I/O data from Telegraf
#[derive(Deserialize, Debug)]
struct DiskIOMetric {
    name: String,
    reads: u64,
    writes: u64,
    read_bytes: u64,
    write_bytes: u64,
    io_time: u64,
}

// Represents Memory data from Telegraf
#[derive(Deserialize, Debug)]
struct MemMetric {
    total: u64,
    available: u64,
    used: u64,
    free: u64,
    used_percent: f64,
}

// Represents Network data from Telegraf
#[derive(Deserialize, Debug)]
struct NetMetric {
    interface: String,
    bytes_sent: u64,
    bytes_recv: u64,
    packets_sent: u64,
    packets_recv: u64,
    err_in: u64,
    err_out: u64,
}

// Represents System data from Telegraf
#[derive(Deserialize, Debug)]
struct SystemMetric {
    hostname: String,
    uptime: u64,
    load1: f64,
    load5: f64,
    load15: f64,
}

// Represents Swap data from Telegraf
#[derive(Deserialize, Debug)]
struct SwapMetric {
    total: u64,
    free: u64,
    used: u64,
    used_percent: f64,
}

// Main structure to receive data as a collection of metrics
#[derive(Deserialize, Debug)]
struct TelegrafData {
    cpu: Vec<CpuMetric>,
    disk: Vec<DiskMetric>,
    diskio: Vec<DiskIOMetric>,
    mem: MemMetric,
    net: Vec<NetMetric>,
    system: SystemMetric,
    swap: SwapMetric,
}

#[derive(Deserialize, Debug)]
pub struct CreateServerStatsRequest {
    pub data: TelegrafData
}


pub fn handle_server_stats(Json(data): Json<CreateServerStatsRequest>)-> impl IntoResponse{
}
