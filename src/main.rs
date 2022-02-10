use futures::executor::block_on;
//use async_timer::Interval;
///Increment Timer
///
///We increment our counter
///For now we just increment counter by 1
///
async fn increment_timer(counter: &mut i32){
    *counter += 1;
}

///Runs our timer and starts jobs
///
///This sets up a dependence on the future provided by increment_timer
///For now we just for increment_timer to return
///
async fn run_timer(counter: &mut i32){
increment_timer(counter).await;
}

fn main() {
    let mut counter: i32 = 0;

    block_on(run_timer(&mut counter));

    println!("Count is: {}", counter);
}
