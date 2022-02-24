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

    //start our Egui window
    let app = TrainSim::new();

    //run window
    run_native(Box::new(app), native_options);
}
