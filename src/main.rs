mod tests;
mod train;
mod trainsim;
mod utils;
mod worker;

use eframe::{run_native, NativeOptions};
use trainsim::TrainSim;

//use train::*;

//Tokio::main macro translates the main function back to a non-async function
// .await calls are transformed to block_on to make for easy coding
#[tokio::main]
async fn main() {
    //default options for Egui
    let native_options = NativeOptions::default();

    //create channels to communicate with worker_loop
    let (ui_transmitter, worker_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (worker_transmitter, ui_receiver) = tokio::sync::mpsc::unbounded_channel();

    //start worker loop
    let _ = tokio::spawn(async {
        worker::worker_loop(worker_receiver, worker_transmitter)
            .await
            .unwrap();
    });

    //start our Egui window
    let app = TrainSim::new(ui_transmitter, ui_receiver);

    //run window
    run_native(Box::new(app), native_options);
}
