use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::utils::ColorHex;
use crate::worker::{self, Command};
use eframe::{
    egui::CentralPanel,
    egui::Color32,
    egui::Context,
    egui::Frame,
    egui::{FontData, FontDefinitions, FontFamily, Style, TextStyle},
    epaint::{FontId, Rounding, Stroke, Vec2},
    epi::App,
};
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

///Application struct used by Egui
///Contains two join handles so worker is kept in scope
/// UI transmitter is used so our struct can send messages to our worker loop
pub struct TrainSim {
    pub count: Arc<AtomicUsize>,
    worker_handle: Option<JoinHandle<()>>,
    join_handle: Option<JoinHandle<()>>,
    ui_transmitter: Option<UnboundedSender<Command>>,
}

//Implement App trait for our struct
impl App for TrainSim {
    //First time setup -> Set fonts, colors etc
    //Create handlers for worker loop and for message passing to our worker
    fn setup(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        //assign custom fonts and styles
        self.configure_fonts(ctx);
        self.configure_style(ctx);

        //create channels to communicate with worker_loop
        let (ui_transmitter, worker_receiver) = tokio::sync::mpsc::unbounded_channel();
        let (worker_transmitter, mut ui_receiver) = tokio::sync::mpsc::unbounded_channel();

        //assign ui transmitter so we can talk to worker loop from our update function
        self.ui_transmitter = Some(ui_transmitter);

        //start worker loop
        self.worker_handle = Some(tokio::spawn(async move {
            worker::worker_loop(worker_receiver, worker_transmitter)
                .await
                .unwrap();
        }));

        //Retrieve weak handles to our data and frame so we can respond if worker loop sends us a message with new data
        let data = self.count.clone();
        let frame_handle = frame.clone();
        self.join_handle = Some(tokio::spawn(async move {
            while let Some(workerdata) = ui_receiver.recv().await {
                data.store(workerdata.count, Ordering::SeqCst);
                frame_handle.request_repaint();
            }
        }));
    }

    ///UI update
    /// This update function is called for each redraw
    /// Here we build our UI using the builder pattern
    /// We use request_repaint() to call this function when needed
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &eframe::epi::Frame) {
        let frame = Frame::none().fill(Color32::from_hex("#3A3C49").unwrap());

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Train Simulator");

                ui.separator();

                ui.add_space(10.0);

                ui.label("Counter");

                ui.add_space(10.0);

                ui.label(self.count.clone().load(Ordering::SeqCst).to_string());
                ui.add_space(10.0);

                if ui.button("reset").clicked() {
                    if let Some(tx) = &self.ui_transmitter {
                        tx.send(Command::Reset).unwrap();
                    }
                }

                ui.add_space(10.0);

                if ui.button("set 5").clicked() {
                    if let Some(tx) = &self.ui_transmitter {
                        tx.send(Command::Counter(5)).unwrap();
                        Context::default().request_repaint();
                    }
                }
            });
        });
    }

    //Set Window title
    fn name(&self) -> &str {
        "Train Simulator"
    }

    //Quit our worker thread upon exiting window
    fn on_exit(&mut self) {
        if let Some(tx) = &self.ui_transmitter {
            tx.send(Command::Quit).unwrap();
        }
        println!("Program is quitting");
    }
}

impl TrainSim {
    pub fn new() -> TrainSim {
        TrainSim {
            count: Arc::new(AtomicUsize::new(0)),
            worker_handle: None,
            join_handle: None,
            ui_transmitter: None,
        }
    }

    ///Configure Fonts
    /// Create Font definitions and sets them into our Context
    /// Assign custom font for the Proportional Font Family
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

    ///Configure Styles
    /// Get's a mutable style Struct from our Context
    /// Adapts some parameters and stores it back
    /// Change colors, sizes for UI controls
    fn configure_style(&self, ctx: &Context) {
        let mut style: Style = (*ctx.style()).clone();

        //Make buttons bigger
        style.spacing.button_padding = Vec2::new(60.0, 20.0);
        style.visuals.widgets.inactive.rounding = Rounding {
            nw: 15.0,
            ne: 15.0,
            sw: 15.0,
            se: 15.0,
        };
        //Change button colors and text colors in fg_stroke
        style.visuals.widgets.inactive.bg_fill = Color32::from_hex("#4e505c").unwrap();
        style.visuals.widgets.inactive.fg_stroke = Stroke {
            width: 1.0,
            color: Color32::from_hex("#FFFFFF").unwrap(),
        };
        style.visuals.widgets.noninteractive.fg_stroke = Stroke {
            width: 1.0,
            color: Color32::from_hex("#FFFFFF").unwrap(),
        };
        //Increase default font sizes for normal widgets and for buttons
        style.text_styles.insert(
            TextStyle::Body,
            FontId {
                size: 30.0,
                family: FontFamily::Proportional,
            },
        );
        style.text_styles.insert(
            TextStyle::Button,
            FontId {
                size: 30.0,
                family: FontFamily::Proportional,
            },
        );
        ctx.set_style(style);
    }
}
