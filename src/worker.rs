
use tokio::time::Instant;
use futures::future::{FutureExt};
use tokio::{select, time::{interval, interval_at}};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;


///Data structure
///This struct holds the data that is passed between worker and UI
#[derive(Debug, Copy, Clone)]
pub struct WorkerData{
    pub count: i32
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

    let mut data = WorkerData{count:0};
    let mut interval = interval(Duration::from_secs(1));

    //let mut now = Instant::now();
    //let mut new_now;

    let channel = t.clone();

    //This loop runs tick and then checks for messages, if there is one, run actions
    loop{
        let m = select!{


            _ = interval.tick() => {
                //Increment counter and update UI until 10
                if data.count < 10 {
                data.count += 1;

                //show time differences between the ticks
                // new_now = Instant::now();
                // println!("{:?}", new_now.checked_duration_since(now));
                // now = Instant::now();

                //update ui
                channel.send(data).unwrap();}
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
                data.count = number;

                //move interval point to round this up to a second
                interval = set_new_interval();
                channel.send(data).unwrap();
            },

            WorkerMessage::Reset => {
                data.count = 0;
                interval = set_new_interval();
                channel.send(data).unwrap();
            },
        }
    }
}

pub fn set_new_interval() -> tokio::time::Interval{
    //move interval point to round this up to a second
    let now = Instant::now();
    let interval = interval_at(now + Duration::from_secs(1), Duration::from_secs(1));
    interval
}

