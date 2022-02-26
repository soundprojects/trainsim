use crate::train::train_structs::*;
use futures::future::FutureExt;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::Instant;
use tokio::{
    select,
    time::{interval, interval_at},
};

///Data structure
///This struct holds the data that is passed between worker and UI
#[derive(Debug, Clone)]
pub struct WorkerData {
    pub count: usize,
    pub train: Train,
    pub track: Track,
}

///Worker Message Enumerator
///For now just contains Quit and Counter for updating our counter from the UI
#[derive(Debug)]
pub enum Command {
    Quit,
    Counter(usize),
    Reset,
}

///Worker loop keeps running our defined tasks until the program is quit
///it allows for the UI to be updated through our weak handle
///sender and receivers are used for callback communication and other signals
pub async fn worker_loop(
    mut r: UnboundedReceiver<Command>,
    t: UnboundedSender<WorkerData>,
) -> tokio::io::Result<()> {
    let mut data = WorkerData {
        count: 0,
        train: Train {
            train_number: 1234,
            train_status: TrainStatus::Stopped,
            train_length: 100,
        },
        track: Track {
            track_length: 1000,
            sections: generate_sections(5, true, 1000),
        },
    };

    let mut data_ref = &mut data;

    let mut interval = interval(Duration::from_secs(1));

    let channel = t.clone();

    //This loop runs tick and then checks for messages, if there is one, run actions
    loop {
        let m = select! {


            _ = interval.tick() => {

                //Run until time = 10
                if data_ref.count < 10 {

                //Increment
                data_ref.count += 1;
                //update train position
                update_train_position(data_ref);

                //update ui
                channel.send(data_ref.clone()).unwrap();}
                continue;
            }

            m = r.recv().fuse() => {

                match m {
                    None => {return Ok(())},
                    Some(m) => m,
                }
            }

        };

        //We got a message from our receiver
        match m {
            Command::Quit => return Ok(()),

            Command::Counter(number) => {
                data_ref.count = number;
                interval = set_new_interval();
                update_train_position(data_ref);
                channel.send(data_ref.clone()).unwrap();
            }

            Command::Reset => {
                data_ref.count = 0;
                interval = set_new_interval();
                update_train_position(data_ref);
                channel.send(data_ref.clone()).unwrap();
            }
        }
    }
}

///Update Train Position
///After each tick we update train position based on elapsed time and if we need to brake for next stop
///For now we use a simple track A --> B, 1000M long, no acceleration or braking yet
/// Train travels 100M / S (crazy fast)
pub fn update_train_position(data: &mut WorkerData) -> &mut WorkerData {
    let mut train = &mut data.train;
    let track = &mut data.track;

    //We move a 100M each tick, back of the train is one train length behind the position if train is not at the end
    let position: usize = train.train_length + (data.count * 100);

    if position > track.track_length {
        println!("Done");
        return data;
    }
    let back_of_train = position - train.train_length;

    //Check if the front or back of the train is one of the sections, that means it is active
    for i in 0..track.sections.len() {
        let section = &mut track.sections[i];
        section.active = false;

        if back_of_train > section.distance_start && back_of_train < section.distance_end {
            section.active = true;
            section.train_number = train.train_number;
        }

        if position > section.distance_start && position < section.distance_end {
            section.active = true;
            section.train_number = train.train_number;
        }
    }

    //If we reached the end of the track, we've stopped
    if position == track.track_length {
        train.train_status = TrainStatus::Stopped;
    } else if back_of_train == 0 {
        train.train_status = TrainStatus::Stopped;
    } else {
        train.train_status = TrainStatus::Running
    }

    data
}

///Set New Interval
///When doing an action where you want to 'restart' counting our interval
///This rounds up to a second before our new tick() returns
pub fn set_new_interval() -> tokio::time::Interval {
    //move interval point to round this up to a second
    let now = Instant::now();
    let interval = interval_at(now + Duration::from_secs(1), Duration::from_secs(1));
    interval
}

///Generate Sections
///This function creates a number of sections within our train track
///A section is either active (a train is running on it) or it is inactive
///Sections have a distance in meters from left to right which we use to determine if our train is on it
///Random will be implemented later to set random distances for each section for more realistic tracks
pub fn generate_sections(amount: usize, random: bool, total_distance: usize) -> Vec<Section> {
    let mut collection: Vec<Section> = Vec::with_capacity(amount);

    let mut current_distance: usize = 0;

    if random {
        for i in 1..amount + 1 {
            let distance_end = i * (total_distance / amount);

            collection.push(Section {
                active: false,
                train_number: 0,
                distance_start: current_distance,
                distance_end: distance_end,
            });

            current_distance = distance_end;
        }
    } else {
        for i in 1..amount + 1 {
            let distance_end = i * (total_distance / amount);

            collection.push(Section {
                active: false,
                train_number: 0,
                distance_start: current_distance,
                distance_end: distance_end,
            });

            current_distance = distance_end;
        }
    }

    collection
}
