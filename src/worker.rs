
use futures::future::{FutureExt};
use tokio::{select, time::{interval}};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use super::*;


///Worker Message Enumerator
///For now just contains Quit and Counter for updating our counter from the UI
#[derive(Debug)]
pub enum WorkerMessage{
    Quit, Counter(i32), Reset,
}

///Worker struct definition
///Channel and thread defined
pub struct Worker{

pub channel: UnboundedSender<WorkerMessage>,
worker_thread: std::thread::JoinHandle<()>
}

///Worker implementation
///implementation creates our worker thread and a channel for communication
impl Worker{

    pub fn new(handle: &MainWindow) -> Self{

        let (channel, r) = tokio::sync::mpsc::unbounded_channel();
        let worker_thread = std::thread::spawn({

            let handle_weak = handle.as_weak();

            move || {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(worker_loop(r, handle_weak))
                    .unwrap()
            }
        });

        Self{
            channel,
            worker_thread
        }

    }

    ///Join is run when the window is closed. We pass a quit message to stop our worker loop from running
    pub fn join(self) -> std::thread::Result<()>{
        let _ = self.channel.send(WorkerMessage::Quit);
        self.worker_thread.join()
    }

}


///Worker loop keeps running our defined tasks until the program is quit
///it allows for the UI to be updated through our weak handle
///sender and receivers are used for callback communication and other signals
async fn worker_loop(mut r: UnboundedReceiver<WorkerMessage>,
    handle: slint::Weak<MainWindow>)
-> tokio::io::Result<()>{

    let mut counter: i32 = 0;
    let mut interval = interval(Duration::from_secs(1));

    //This loop runs tick and then checks for messages, if there is one, run actions
    loop{
        let m = select!{


            _ = interval.tick() => {
                //Increment counter and update UI until 10
                if counter < 10 {
                counter += 1;
                handle.clone().upgrade_in_event_loop(move |h|
                h.set_counter(SharedString::from(counter.to_string())));}
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
                handle.clone()
                .upgrade_in_event_loop(move |h|
                h.set_counter(SharedString::from(counter.to_string()))
                );
            },

            WorkerMessage::Reset => {
                counter = 0;
                handle.clone()
                .upgrade_in_event_loop(move |h|
                h.set_counter(SharedString::from(counter.to_string()))
                );
            },
        }
    }
}

