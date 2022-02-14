use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use super::*;

pub enum WorkerMessage{
    Quit, Counter(i32)
}

pub struct Worker{

pub channel: UnboundedSender<WorkerMessage>,
worker_thread: std::thread::JoinHandle<()>
}


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

    pub fn join(self) -> std::thread::Result<()>{
        let _ = self.channel.send(WorkerMessage::Quit);
        self.worker_thread.join()
    }

}

async fn worker_loop(mut _r: UnboundedReceiver<WorkerMessage>,
    handle: slint::Weak<MainWindow>)
-> tokio::io::Result<()>{

    let mut counter: i32 = 0;
    let mut interval = Interval::platform_new(core::time::Duration::from_secs(1));

//for now we wait for 10 seconds to complete
    while counter < 10
    {

        //increment
        counter += 1;
        println!("Counter is {}", counter);

        //update our counter in the UI
        handle.clone().upgrade_in_event_loop(move |h| h.set_counter(SharedString::from(counter.to_string())));

        //wait a second
        interval.wait().await;
    }

    Ok(())

}
