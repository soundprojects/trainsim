use async_timer::Interval;


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
///This sets up a dependency on the future provided by increment_timer
///For now we just run increment_timer for 5 seconds using the Interval
///
async fn run_timer(counter: &mut i32){
    let mut times: i32 = 0;
    let mut interval = Interval::platform_new(core::time::Duration::from_secs(1));

//for now we wait for 5 seconds to complete
    while times < 5
    {

        increment_timer(counter).await;
        interval.wait().await;
        times += 1;
    }
}

//Tokio::main macro translates the main function back to a non-async function
// .await calls are transformed to block_on to make for easy coding
#[tokio::main]
async fn main() {
    let mut counter: i32 = 0;

    let task = tokio::spawn(async move {
        run_timer(&mut counter).await;
        println!("Count is: {}", counter);
    });

    //block on our task
    let _result = task.await.unwrap(); 

}
