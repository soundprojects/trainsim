
use futures::future::{FutureExt};
use tokio::{select, time::{interval}};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;


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
    t: UnboundedSender<i32>)
-> tokio::io::Result<()>{

    let mut counter: i32 = 0;
    let mut interval = interval(Duration::from_secs(1));

    let channel = t.clone();

    //This loop runs tick and then checks for messages, if there is one, run actions
    loop{
        let m = select!{


            _ = interval.tick() => {
                //Increment counter and update UI until 10
                if counter < 10 {
                counter += 1;
                println!("tick");
                channel.send(counter).unwrap();}
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
                counter = number;
                channel.send(counter).unwrap();
            },

            WorkerMessage::Reset => {
                counter = 0;
                channel.send(counter).unwrap();
            },
        }
    }
}

