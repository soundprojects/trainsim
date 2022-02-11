use slint::SharedString;
use async_timer::Interval;


///Runs our timer and starts jobs
///
///This runs a timer at a 1s interval and updates the UI
///For now we just run this for 10 seconds
///
async fn run_timer(handle: slint::Weak<MainWindow>){
    
    let mut counter: i32 = 0;
    let mut times: i32 = 0;
    let mut interval = Interval::platform_new(core::time::Duration::from_secs(1));

//for now we wait for 10 seconds to complete
    while times < 10
    {

        //increment
        counter += 1;
        println!("Counter is {}", counter);

        //update our counter in the UI
        handle.clone().upgrade_in_event_loop(move |h| h.set_counter(SharedString::from(counter.to_string())));

        //wait a second
        interval.wait().await;
        times += 1;
    }
}


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

    //start timer task and pass our window
    let task = tokio::spawn(async move {
        run_timer(handle).await;
        
    });


    //run window
    window.run();

    //block on our timer task
    let _result = task.await.unwrap(); 

    //if we got here, timer is done and window is closed
    println!("Program is quitting");
}

