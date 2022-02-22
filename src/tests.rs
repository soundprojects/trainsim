#[cfg(test)]
use crate::worker::*;
#[cfg(test)]
use crate::train::train_structs::*;
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
            let item = &collection[i];
            assert!(item.distance_start < item.distance_end);
            assert!(item.distance_end > 0 );
            assert!(item.distance_end <= 1000);
            assert_eq!(item.active, false);
        }
    }


///Test Update Train Position
///Train position and back of train should be positive values, within track length where position is a train length larger than back of train
///We check that the train occupies the sections which are occupied with front or back of train and that all others are inactive
///We check for correct train status at start, running and at the end of the track
///Count = 0, train occupies first section, status = stopped
///[---------|----------]
///[|||||----|----------]
///Count = 1, train occupies first and second section, status = running
///[---------|----------]
///[------|||||||-------]
///Count = 2, train occupies second section, status = stopped
///[---------|----------]
///[---------|----||||||]
///Count = 3 should be the same as previous so check for no changes
#[test]
    fn test_update_train_position(){
    
    //Create our test data
    let mut data = WorkerData{count:0,
        train: Train{train_number:1, train_status: TrainStatus::Stopped, train_length: 100},
        track: Track{track_length:300,
                sections:generate_sections(2, true, 300)
            }
        };

    //Go to next update so our sections are properly active 
    update_train_position(&mut data);

    //Count = 0
    assert!(matches!(data.train.train_status, TrainStatus::Stopped));

    assert_eq!(data.track.sections[0].active, true);
    assert_eq!(data.track.sections[1].active, false);


    //Increment and go to next update
    data.count +=1 ;
    update_train_position(&mut data);

    //Count = 1
    assert!(matches!(data.train.train_status, TrainStatus::Running));

    assert_eq!(data.track.sections[0].active, true);
    assert_eq!(data.track.sections[1].active, true);

    //Go to final update
    data.count +=1 ;
    update_train_position(&mut data);

    //Count = 2
    assert!(matches!(data.train.train_status, TrainStatus::Stopped));

    assert_eq!(data.track.sections[0].active, false);
    assert_eq!(data.track.sections[1].active, true);

    //Check for same status
    data.count +=1 ;
    update_train_position(&mut data);

    //Count = 3
    assert!(matches!(data.train.train_status, TrainStatus::Stopped));

    assert_eq!(data.track.sections[0].active, false);
    assert_eq!(data.track.sections[1].active, true);


    }