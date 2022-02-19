#[cfg(test)]
use std::time::Duration;
#[cfg(test)]
use tokio::time::Instant;
#[cfg(test)]
use super::*;


///Test Set Interval
///Set interval creates a new interval and should make the time between the previous instant and the new tick between 900 - 1100 ms
#[tokio::test]
async fn test_set_interval() {
    let now = Instant::now();
    let mut interval = worker::set_new_interval();
    interval.tick().await;
    let new_now = Instant::now();
    assert!(new_now.checked_duration_since(now).unwrap() > Duration::from_millis(900)
    && new_now.checked_duration_since(now).unwrap() < Duration::from_millis(1100));
}


///Test Generate Sections
///Sections should always have a distance larger than 0 and no larger than the maximum distance
///Check item count as well
#[test]
    fn test_generate_sections(){
        let collection = worker::generate_sections(2, true, 1000);

        assert_eq!(collection.len(), 2);

        for i in 0..2{
            assert!(collection[i].distance > 0 );
            assert!(collection[i].distance <= 1000);
            assert_eq!(collection[i].active, false);
        }
    }