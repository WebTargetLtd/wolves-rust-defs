
#[cfg(feature = "serde_support")]
use serde::{Serialize, Deserialize};
use std::fmt::Debug;



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
