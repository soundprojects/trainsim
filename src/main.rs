use crate::worker::worker_loop;
use slint::SharedString;

mod worker;

//Tokio::main macro translates the main function back to a non-async function
// .await calls are transformed to block_on to make for easy coding
//Slint include modules allows use to import our external slint files using build.rs
slint::include_modules!();
#[tokio::main]
async fn main() {
    //start our Slint Window
    let window = MainWindow::new();

    //start worker task and pass our window handle
    // let worker = worker::Worker::new(&window);

    let (channel, r) = tokio::sync::mpsc::unbounded_channel();

    //assign callbacks
    let clone = channel.clone();
    window.on_reset({
        // let channel = worker.channel.clone();
        move || {
            clone.send(worker::WorkerMessage::Reset).unwrap();
        }
    });

    window.on_set_counter({
        // let channel = worker.channel.clone();
        move |number| {
            channel
                .send(worker::WorkerMessage::Counter(number))
                .unwrap();
        }
    });

    let handle_weak = window.as_weak();

    let mut worker = tokio::spawn(async move {
        worker_loop(r, handle_weak).await;
    });

    //run window
    window.run();

    // If we got here, worker window is closed, so we join worker thread
    println!("Clean exit");
}
