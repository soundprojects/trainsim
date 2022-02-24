use crate::utils::ColorHex;
use crate::worker::{WorkerData, WorkerMessage};
use eframe::{
    egui::CentralPanel,
    egui::Color32,
    egui::Context,
    egui::Frame,
    egui::{FontData, FontDefinitions, FontFamily, Style, TextStyle},
    epaint::{FontId, Rounding, Stroke, Vec2},
    epi::App,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

//Application struct used by Egui. Contains Tx/Rx for communicating with Worker Loop
pub struct TrainSim {
    pub count: usize,
    pub ui_transmitter: UnboundedSender<WorkerMessage>,
    pub ui_receiver: UnboundedReceiver<WorkerData>,
}

//Implement App trait for our struct
impl App for TrainSim {
    //First time setup -> Set fonts, colors etc
    fn setup(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    //UI update
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

        let frame = Frame::none().fill(Color32::from_hex("#3A3C49").unwrap());

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Train Simulator");

                ui.separator();

                ui.add_space(10.0);

                ui.label("Counter");

                ui.add_space(10.0);

                ui.label(self.count.to_string());

                ui.add_space(10.0);

                if ui.button("reset").clicked() {
                    self.ui_transmitter.send(WorkerMessage::Reset).unwrap();
                }

                ui.add_space(10.0);

                if ui.button("set 5").clicked() {
                    self.ui_transmitter.send(WorkerMessage::Counter(5)).unwrap();
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
        self.ui_transmitter.send(WorkerMessage::Quit).unwrap();
        println!("Program is quitting");
    }
}

impl TrainSim {
    pub fn new(
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

        let mut style: Style = (*ctx.style()).clone();

        style.spacing.button_padding = Vec2::new(60.0, 20.0);
        style.visuals.widgets.inactive.rounding = Rounding {
            nw: 15.0,
            ne: 15.0,
            sw: 15.0,
            se: 15.0,
        };
        style.visuals.widgets.inactive.bg_fill = Color32::from_hex("#4e505c").unwrap();
        style.visuals.widgets.inactive.fg_stroke = Stroke {
            width: 1.0,
            color: Color32::from_hex("#FFFFFF").unwrap(),
        };
        style.visuals.widgets.noninteractive.fg_stroke = Stroke {
            width: 1.0,
            color: Color32::from_hex("#FFFFFF").unwrap(),
        };
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
