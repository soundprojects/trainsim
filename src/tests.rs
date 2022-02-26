#[cfg(test)]
use super::*;
#[cfg(test)]
use crate::train::train_structs::*;
#[cfg(test)]
use crate::utils::*;
#[cfg(test)]
use crate::worker::*;
#[cfg(test)]
use eframe::epaint::Color32;
#[cfg(test)]
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
    assert!(
        new_now.checked_duration_since(now).unwrap() > Duration::from_millis(900)
            && new_now.checked_duration_since(now).unwrap() < Duration::from_millis(1100)
    );
}

///Test Generate Sections
///Sections should always have a distance larger than 0 and no larger than the maximum distance
///Check item count as well
#[test]
fn test_generate_sections() {
    let collection = worker::generate_sections(2, true, 1000);

    assert_eq!(collection.len(), 2);

    for i in 0..2 {
        let item = &collection[i];
        assert!(item.distance_start < item.distance_end);
        assert!(item.distance_end > 0);
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
//#[test]
// fn test_update_train_position() {
//     //Create our test data
//     let mut data = WorkerData {
//         count: 0,
//         train_system: TrainSystem::new(),
//     };

//     //Go to next update so our sections are properly active
//     update_train_position(&mut data);

//     //Count = 0
//     assert!(matches!(data.train.train_status, TrainStatus::Stopped));

//     assert_eq!(data.track.sections[0].active, true);
//     assert_eq!(data.track.sections[1].active, false);

//     //Increment and go to next update
//     data.count += 1;
//     update_train_position(&mut data);

//     //Count = 1
//     assert!(matches!(data.train.train_status, TrainStatus::Running));

//     assert_eq!(data.track.sections[0].active, true);
//     assert_eq!(data.track.sections[1].active, true);

//     //Go to final update
//     data.count += 1;
//     update_train_position(&mut data);

//     //Count = 2
//     assert!(matches!(data.train.train_status, TrainStatus::Stopped));

//     assert_eq!(data.track.sections[0].active, false);
//     assert_eq!(data.track.sections[1].active, true);

//     //Check for same status
//     data.count += 1;
//     update_train_position(&mut data);

//     //Count = 3
//     assert!(matches!(data.train.train_status, TrainStatus::Stopped));

//     assert_eq!(data.track.sections[0].active, false);
//     assert_eq!(data.track.sections[1].active, true);
// }

///Test Train System Struct
///This struct does some manual adding to hashmaps for structures pins and connections
/// Check that the assignments correspond to the idx's generated for the structures and they match up with pin idx's in our hashmaps
#[test]
fn test_train_system_struct() {
    let mut system = TrainSystem::new();

    system.track(1000);

    assert_eq!(system.structures.len(), 1);
    assert_eq!(system.pins.len(), 2);

    if let Some(structure) = system.structures.get(&0) {
        if let StructureTypes::Track(track) = &structure {
            assert_eq!(track.idx, 0);
            assert_eq!(track.input_pin, 0);
            assert_eq!(track.output_pin, 1);
        }
    }

    // now we have 1 structure with idx 0, input pin idx 0 and output pin idx 1
    //check that switch becomes idx 1, input pin idx 2, left output 3 and right output 4
    system.switch(SwitchType::LeftSplitDown);

    assert_eq!(system.structures.len(), 2);
    assert_eq!(system.pins.len(), 5);

    if let Some(structure) = system.structures.get(&1) {
        if let StructureTypes::Switch(switch) = &structure {
            assert_eq!(switch.idx, 1);
            assert_eq!(switch.input_pin, 2);
            assert_eq!(switch.left_output_pin, 3);
            assert_eq!(switch.right_output_pin, 4);
        }
    }
}

///Color Test
/// Tests Utility function to convert a hex string to a egui Color 32
#[test]
fn test_color() {
    let blue = Color32::BLUE;
    let hex = blue.to_hex();
    assert_eq!(hex, "#0000ff");
    assert_eq!(Color32::BLUE, Color32::from_hex_panic(&hex));
}
