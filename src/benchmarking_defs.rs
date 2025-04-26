
#[cfg(feature = "serde_support")]
use serde::{Serialize, Deserialize};
use std::{collections::BTreeMap, fmt::Debug, net::IpAddr};

use crate::sysinfo_defs::SystemInfo;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
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
pub struct FlatBenchmarks {
    pub hostname: String,
    pub ip: IpAddr,
    pub benchmark: Benchmark
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize, Clone))]
pub struct FullSystemInfo {
    pub hostname: String,
    pub app_version: Option<String>,
    pub ip: Option<IpAddr>,
    pub source_path: Option<String>,
    pub benchmarks: Vec<Benchmark>,
    pub sysinfo: SystemInfo,
}
impl FullSystemInfo {

    pub fn get_all_benchmarks_sorted_by_result(endpoints: &[FullSystemInfo]) -> Vec<Benchmark> {
        let mut all_benchmarks: Vec<Benchmark> = endpoints
            .iter()
            .flat_map(|endpoint| endpoint.benchmarks.clone())
            .collect();

        all_benchmarks.sort_by(|a, b| a.result.partial_cmp(&b.result).unwrap_or(std::cmp::Ordering::Equal));
        all_benchmarks
    }

    pub fn to_flat_benchmarks(endpoints: &[FullSystemInfo]) -> BTreeMap<u32, FlatBenchmarks> {
        let mut flat_benchmarks_map: BTreeMap<u32, FlatBenchmarks> = BTreeMap::new();

        endpoints.iter().for_each(|endpoint| {
            endpoint.benchmarks.iter().for_each(|benchmark| {
                // Insert into the BTreeMap with the result as the key
                flat_benchmarks_map.insert(
                    -benchmark.result as u32, // Use negative result to reverse the order (largest first)
                    FlatBenchmarks {
                        hostname: endpoint.hostname.clone(),
                        ip: endpoint.ip.unwrap(), // Assuming `SystemInfo` has an `ip` field
                        benchmark: benchmark.clone(),
                    },
                );
            });
        });

        flat_benchmarks_map
    }
    
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
