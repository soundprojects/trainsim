use super::*;
use std::time::Duration;
#[cfg(test)]
use tokio::time::Instant;



///Test Set Interval
///Set interval creates a new interval and should make the time between the previous instant and the new tick between 900 - 1100 ms
#[tokio::test]
async fn test_set_interval() {
    let now = Instant::now();
    let mut interval = worker::set_new_interval();
    interval.tick().await;
    let new_now = Instant::now();
    assert!(new_now.checked_duration_since(now).unwrap() > Duration::from_millis(900) && new_now.checked_duration_since(now).unwrap() < Duration::from_millis(1100));
}
