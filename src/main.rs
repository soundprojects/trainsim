use eframe::egui;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
mod tests;
mod train;
mod utils;
mod worker;

use eframe::{
    egui::CentralPanel,
    egui::Context,
    egui::{FontData, FontDefinitions},
    epaint::FontFamily,
    epi::App,
    run_native, NativeOptions,
};
use utils::ColorHex;
use worker::WorkerData;
use worker::WorkerMessage;

//use train::*;
struct TrainSim {
    pub count: usize,
    pub ui_transmitter: UnboundedSender<WorkerMessage>,
    pub ui_receiver: UnboundedReceiver<WorkerData>,
}

impl App for TrainSim {
    fn setup(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &eframe::epi::Frame) {
        //Redraw so incoming messages also update the UI, not just widgets/mouse
        ctx.request_repaint();

        //Check for messages from our worker loop <-- This is called many times, can we do this differently?
        match self.ui_receiver.try_recv() {
            Ok(data) => {
                self.count = data.count;
            }
            Err(_e) => {
                //println!("Error receiving message {}", e);
            }
        };

        let frame = egui::Frame::none().fill(egui::Color32::from_hex("#3A3C49").unwrap());
        CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.label(self.count.to_string());

            if ui.button("reset").clicked() {
                self.ui_transmitter.send(WorkerMessage::Reset).unwrap();
            }

            if ui.button("set 5").clicked() {
                self.ui_transmitter.send(WorkerMessage::Counter(5)).unwrap();
            }
        });
    }

    fn name(&self) -> &str {
        "Train Simulator"
    }

    fn on_exit(&mut self) {
        self.ui_transmitter.send(WorkerMessage::Quit).unwrap();
        println!("Program is quitting");
    }
}

impl TrainSim {
    fn new(
        ui_transmitter: UnboundedSender<WorkerMessage>,
        ui_receiver: UnboundedReceiver<WorkerData>,
    ) -> TrainSim {
        TrainSim {
            count: 0,
            ui_transmitter: ui_transmitter,
            ui_receiver: ui_receiver,
        }
    }

    fn configure_fonts(&self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();

        font_def.font_data.insert(
            "Avenir".to_string(),
            FontData::from_static(include_bytes!("../ui/fonts/Avenir_85_Heavy.ttf")),
        );

        font_def
            .families
            .insert(FontFamily::Proportional, vec!["Avenir".to_owned()]);

        ctx.set_fonts(font_def);
    }
}

//Tokio::main macro translates the main function back to a non-async function
// .await calls are transformed to block_on to make for easy coding
#[tokio::main]
async fn main() {
    let native_options = NativeOptions::default();
    //create channels to communicate with worker_loop
    let (ui_transmitter, worker_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (worker_transmitter, ui_receiver) = tokio::sync::mpsc::unbounded_channel();

    let _ = tokio::spawn(async {
        worker::worker_loop(worker_receiver, worker_transmitter)
            .await
            .unwrap();
    });

    //start our Egui window
    let app = TrainSim::new(ui_transmitter, ui_receiver);

    //run window
    run_native(Box::new(app), native_options);

    // channel.send(worker::WorkerMessage::Quit).unwrap();
    // join_handle.await.unwrap();
}
