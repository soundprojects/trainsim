
use futures::future::{FutureExt};
use tokio::{select, time::{interval}};
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

    let channel = t.clone();

    //This loop runs tick and then checks for messages, if there is one, run actions
    loop{
        let m = select!{


            _ = interval.tick() => {
                //Increment counter and update UI until 10
                if data.count < 10 {
                data.count += 1;
                println!("tick");
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
                channel.send(data).unwrap();
            },

            WorkerMessage::Reset => {
                data.count = 0;
                channel.send(data).unwrap();
            },
        }
    }
}

