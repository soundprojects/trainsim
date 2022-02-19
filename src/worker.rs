
use tokio::time::Instant;
use futures::future::{FutureExt};
use tokio::{select, time::{interval, interval_at}};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;


///Data structure
///This struct holds the data that is passed between worker and UI
#[derive(Debug, Clone)]
pub struct WorkerData{
    pub count: i32,
    pub train: Train,
    pub track: Track
}

///Train structure
///This struct holds the data that is passed between worker and UI
#[derive(Debug, Copy, Clone)]
pub struct Train{
    pub train_number: i32,
    pub train_status: TrainStatus,
    pub train_length: i32
}

///Train Track
///Struct to hold a simple train track
#[derive(Debug, Clone)]
pub struct Track{
    pub origin: String,
    pub destination: String,
    pub track_length: i32,
    pub sections: Vec<Section>
}

#[derive(Debug, Clone)]
pub struct Section{
    pub active :bool,
    pub train_number :i32,
    pub distance :usize
}

///Train Status Enumerator
///Status indicators for driving braking etc
#[derive(Debug, Copy, Clone)]
pub enum TrainStatus{
    Stopped, Running
}

///Worker Message Enumerator
///For now just contains Quit and Counter for updating our counter from the UI
#[derive(Debug)]
pub enum WorkerMessage{
    Quit, Counter(i32), Reset,
}

///Worker loop keeps running our defined tasks until the program is quit
///it allows for the UI to be updated through our weak handle
///sender and receivers are used for callback communication and other signals
pub async fn worker_loop(mut r: UnboundedReceiver<WorkerMessage>,
    t: UnboundedSender<WorkerData>)
-> tokio::io::Result<()>{

    let mut data = WorkerData{count:0,
    train: Train{train_number:1234, train_status: TrainStatus::Stopped, train_length: 100},
    track: Track{origin: "A".to_string(),
                destination:"B".to_string(),
                track_length:1000,
                sections:generate_sections(5, true, 1000)
            }
    };

    let mut data_ref = &mut data;

    let mut interval = interval(Duration::from_secs(1));

    let channel = t.clone();

    //This loop runs tick and then checks for messages, if there is one, run actions
    loop{
        let m = select!{


            _ = interval.tick() => {

                //Run until time = 10
                if data_ref.count < 10 {

                //update train position
                update_train_position(data_ref);

                //update ui
                channel.send(data_ref.clone()).unwrap();}
                continue;
            }

            m = r.recv().fuse() => {
                match m {
                    None => return Ok(()),
                    Some(m) => m,
                }
            }

        };

    //We got a message from our receiver
    match m{
            WorkerMessage::Quit => return Ok(()),

            WorkerMessage::Counter(number) => {
                data_ref.count = number;
                interval = set_new_interval();
                channel.send(data_ref.clone()).unwrap();
            },

            WorkerMessage::Reset => {
                data_ref.count = 0;
                interval = set_new_interval();
                channel.send(data_ref.clone()).unwrap();
            },
        }
    }
}

///Update Train Position
///After each tick we update train position based on elapsed time and if we need to brake for next stop
///For now we use a simple track A --> B, 1000M long, no acceleration or braking yet
/// Train travels 100M / S (crazy fast)
pub fn update_train_position(data: &mut WorkerData) -> &mut WorkerData{

    let mut train = data.train;

    //First tick? Start train
    data.count += 1;
    train.train_status = TrainStatus::Running;

    data
}


///Set New Interval
///When doing an action where you want to 'restart' counting our interval
///This rounds up to a second before our new tick() returns
pub fn set_new_interval() -> tokio::time::Interval{
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
pub fn generate_sections(amount: usize, random: bool, total_distance: usize) -> Vec<Section>{

    let mut collection :Vec<Section> = Vec::with_capacity(amount);

    if random{
        let _distance :usize = 0;
        for i in 1..amount+1{
            collection.push(Section{active:false, train_number: 0, distance: (i * (total_distance / amount))})
        }
    }
    else{
        for i in 1..amount+1{
            collection.push(Section{active:false, train_number: 0, distance: (i * (total_distance / amount))})
        }
    }

    collection
}
