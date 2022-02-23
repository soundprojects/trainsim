mod worker;
mod tests;
mod train;

use eframe::{egui::CentralPanel, epi::App, run_native};

//use train::*;
struct TrainSim{}

impl App for TrainSim{

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) { 

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hiya");

        });

    }


    fn name(&self) -> &str { 

        "Train Simulator"

    }
}


//Tokio::main macro translates the main function back to a non-async function
// .await calls are transformed to block_on to make for easy coding
#[tokio::main]
async fn main() {
    
    //start our Egui window

    //create channels to communicate with worker_loop
    let (channel, r) = tokio::sync::mpsc::unbounded_channel();
    let (t, mut receiver) = tokio::sync::mpsc::unbounded_channel();

    let join_handle = tokio::spawn(async {
        worker::worker_loop(r, t).await.unwrap();});

    let _recv_handle = tokio::spawn(async move {
        while let Some(_workerdata) = receiver.recv().await{
            
            //update UI

    }});

    //assign callbacks



    //run window
    run_native(app, native_options);

    //window is closed, quit worker loop
    channel.send(worker::WorkerMessage::Quit).unwrap();
    join_handle.await.unwrap();


    println!("Program is quitting");


}

