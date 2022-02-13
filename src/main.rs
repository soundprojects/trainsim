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
    let task = tokio::spawn(async move {
        worker::run_timer(handle).await;
        
    });

    //run window
    window.run();

    //block on our worker task
    let _result = task.await.unwrap(); 

    //if we got here, worker is done and window is closed
    println!("Program is quitting");
}

