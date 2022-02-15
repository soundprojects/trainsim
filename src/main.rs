mod worker;


//Tokio::main macro translates the main function back to a non-async function
// .await calls are transformed to block_on to make for easy coding
//Slint include modules allows use to import our external slint files using build.rs
slint::include_modules!();
#[tokio::main]
async fn main() {
    
    //start our Slint Window
    let window = MainWindow::new();
    let handle = window.as_weak();

    //create channels to communicate with worker_loop
    let (channel, r) = tokio::sync::mpsc::unbounded_channel();
    let (t, mut receiver) = tokio::sync::mpsc::unbounded_channel();

    let join_handle = tokio::spawn(async {
        worker::worker_loop(r, t).await.unwrap();});

    let _recv_handle = tokio::spawn(async move {
        while let Some(counter) = receiver.recv().await{
            
            //update UI
            handle.clone()
                .upgrade_in_event_loop(move |h|
                h.set_counter(counter)
                );
            }
    });

    //assign callbacks
    window.on_reset({
        let channel = channel.clone();
        move || {
        channel.send(worker::WorkerMessage::Reset).unwrap();
        }
    });

    window.on_set_counter({
        let channel = channel.clone();
        move |number| {
        channel.send(worker::WorkerMessage::Counter(number)).unwrap();
        }
    });

    //run window
    window.run();

    //window is closed, quit worker loop
    channel.send(worker::WorkerMessage::Quit).unwrap();
    join_handle.await.unwrap();


    println!("Program is quitting");


}

