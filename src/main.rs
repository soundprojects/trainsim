mod worker;
mod tests;


//Convert between our Section and the slint Section types <-- Ugly stuff
impl From<worker::Section> for slint_generatedMainWindow::Section{
    fn from(section: worker::Section) -> Self{

        slint_generatedMainWindow::Section{
            active: section.active,
            train_number: section.train_number as i32,
            distance_start: section.distance_start as i32,
            distance_end: section.distance_end as i32}

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
    let handle = window.as_weak();

    //create channels to communicate with worker_loop
    let (channel, r) = tokio::sync::mpsc::unbounded_channel();
    let (t, mut receiver) = tokio::sync::mpsc::unbounded_channel();

    let join_handle = tokio::spawn(async {
        worker::worker_loop(r, t).await.unwrap();});

    let _recv_handle = tokio::spawn(async move {
        while let Some(workerdata) = receiver.recv().await{
            
            //update UI
            handle.clone()
                .upgrade_in_event_loop(move |h| {
                    h.set_counter(workerdata.count.try_into().unwrap());

                    let mut sections :Vec<slint_generatedMainWindow::Section> = Vec::new();
                    sections.extend(workerdata.track.sections.into_iter().map(From::from));

                    h.set_sections(std::rc::Rc::new(slint::VecModel::from(sections)).into());
                    }
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
        channel.send(worker::WorkerMessage::Counter(number.try_into().unwrap())).unwrap();
        }
    });


    //run window
    window.run();

    //window is closed, quit worker loop
    channel.send(worker::WorkerMessage::Quit).unwrap();
    join_handle.await.unwrap();


    println!("Program is quitting");


}

