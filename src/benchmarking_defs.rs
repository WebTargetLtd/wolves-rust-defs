
#[cfg(feature = "serde_support")]
use serde::{Serialize, Deserialize};
use std::fmt::Debug;

use crate::sysinfo_defs::SystemInfo;

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize, Clone))]
pub struct Benchmark {
    pub cores: usize,
    pub result: f64,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize, Clone))]
pub struct Benchmarks {
    pub hostname: String,
    pub source_path: Option<String>,
    pub benchmarks: Vec<Benchmark>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize, Clone))]
pub struct FullSystemInfo {
    pub hostname: String,
    pub source_path: Option<String>,
    pub benchmarks: Vec<Benchmark>,
    pub sysinfo: SystemInfo,
}
impl FullSystemInfo {
    pub fn find_min_max_benchmark(&self) -> Option<(Benchmark, Benchmark)> {
        if self.benchmarks.is_empty() {
            return None;
        }

        let min_result = self
            .benchmarks
            .iter()
            .min_by(|a, b| a.result.partial_cmp(&b.result).unwrap_or(std::cmp::Ordering::Equal))
            .map(|b| Benchmark { cores: b.cores, result: b.result } )?;

        let max_result = self
            .benchmarks
            .iter()
            .max_by(|a, b| a.result.partial_cmp(&b.result).unwrap_or(std::cmp::Ordering::Equal))
            .map(|b| Benchmark { cores: b.cores, result: b.result } )?;

        Some((min_result, max_result))
    }
}
