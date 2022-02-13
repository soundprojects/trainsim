use super::*;

///Runs our timer and starts jobs
///
///This runs a timer at a 1s interval and updates the UI
///For now we just run this for 10 seconds
///
pub async fn run_timer(handle: slint::Weak<MainWindow>){
    
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
}