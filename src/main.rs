use slint::SharedString;
use async_timer::Interval;


mod worker;


//Tokio::main macro translates the main function back to a non-async function
// .await calls are transformed to block_on to make for easy coding
//Slint include modules allows use to import our external slint files using build.rs
slint::include_modules!();
#[tokio::main]
async fn main() {

    //start our Slint Window
    let window = MainWindow::new();

    //get reference to our window that we can send across threads
    let handle = window.as_weak();

    //start worker task and pass our window handle
    let worker = worker::Worker::new(&window);


    //run window
    window.run();


    //if we got here, worker window is closed, so we join worker thread
    println!("Program is quitting");

    worker.join().unwrap();
}

