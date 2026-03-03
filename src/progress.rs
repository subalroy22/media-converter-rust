use regex::Regex;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ProgressTracker {
    pub total_duration: Arc<Mutex<Option<f64>>>,
}

impl ProgressTracker
{
    pub fn new() -> Self {
        ProgressTracker {
            total_duration: Arc::new(Mutex::new(None)),
        }
    }

    pub fn parse_duration(line: &str) -> Option<f64> {
        let re = Regex::new(r"Duration: (\d{2}):(\d{2}):(\d{2}\.\d{2})").ok()?;
        let caps = re.captures(line)?;
        
        let hours: f64 = caps.get(1)?.as_str().parse().ok()?;
        let minutes: f64 = caps.get(2)?.as_str().parse().ok()?;
        let seconds: f64 = caps.get(3)?.as_str().parse().ok()?;
        
        Some(hours * 3600.0 + minutes * 60.0 + seconds)
    }

    pub fn parse_time(line: &str) -> Option<f64> {
        let re = Regex::new(r"time=(\d{2}):(\d{2}):(\d{2}\.\d{2})").ok()?;
        let caps = re.captures(line)?;
        
        let hours: f64 = caps.get(1)?.as_str().parse().ok()?;
        let minutes: f64 = caps.get(2)?.as_str().parse().ok()?;
        let seconds: f64 = caps.get(3)?.as_str().parse().ok()?;
        
        Some(hours * 3600.0 + minutes * 60.0 + seconds)
    }

    pub async fn calculate_progress(&self, current_time: f64) -> f32 {
        let duration = self.total_duration.lock().await;
        if let Some(total) = *duration {
            if total > 0.0 {
                return ((current_time / total) * 100.0).min(100.0) as f32;
            }
        }
        0.0
    }
}
