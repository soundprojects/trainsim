use std::collections::HashMap;
use std::hash::Hash;

use eframe::egui::output;

use crate::worker::generate_sections;

///Train structure
///This struct holds the data that is passed between worker and UI
#[derive(Debug, Copy, Clone)]
pub struct Train {
    pub train_number: i32,
    pub train_status: TrainStatus,
    pub train_length: usize,
}

///Train Track
///Struct to hold a simple train track
#[derive(Debug, Clone)]
pub struct Track {
    pub track_length: usize,
    pub sections: Vec<Section>,
    pub input_pin: usize,
    pub output_pin: usize,
    pub idx: usize,
}

///Switch
///Struct to hold a simple switch
#[derive(Debug, Clone)]
pub struct Switch {
    pub sections: Vec<Section>,
    pub switch_type: SwitchType,
    pub input_pin: usize,
    pub left_output_pin: usize,
    pub right_output_pin: usize,
    pub idx: usize,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub active: bool,
    pub train_number: i32,
    pub distance_start: usize,
    pub distance_end: usize,
}

///Train Status Enumerator
///Status indicators for driving braking etc
#[derive(Debug, Copy, Clone)]
pub enum TrainStatus {
    Stopped,
    Running,
}

///Switch Type Enumerator
///switch types and directions
#[derive(Debug, Copy, Clone)]
pub enum SwitchType {
    LeftSplitUp,
    LeftSplitDown,
    RightSplitUp,
    RightSplitDown,
}

pub trait Identifiable {
    fn get_idx(&self) -> usize;
}

#[derive(Debug, Clone)]
pub enum StructureTypes {
    Track(Track),
    Switch(Switch),
}

impl Identifiable for StructureTypes {
    fn get_idx(&self) -> usize {
        match &self {
            &StructureTypes::Track(track) => track.idx,
            &StructureTypes::Switch(switch) => switch.idx,
        }
    }
}

///Train System Struct
/// A train system consists of tracks, switches or stations. They are all contained in the structures HashMap
/// Each structure has the trait identifiable because they all have a unique 'idx'
/// Each structure has a number of pins, either input or output, the idx's are all stored in the pins hashmap
/// Connections contain the connections between the structures pin's
#[derive(Debug, Clone, Default)]
pub struct TrainSystem {
    pub pins: HashMap<usize, usize>,
    pub connections: HashMap<usize, usize>,
    pub structures: HashMap<usize, StructureTypes>,
}

impl TrainSystem {
    pub fn new() -> Self {
        TrainSystem::default()
    }

    pub fn track(&mut self, length: usize) -> &Self {
        let idx = self.structures.len();

        let input_idx = self.pins.len();
        self.pins.insert(input_idx, idx);

        let output_idx = self.pins.len();
        self.pins.insert(output_idx, idx);

        let track = StructureTypes::Track(Track {
            track_length: length,
            sections: generate_sections(5, false, length),
            input_pin: input_idx,
            output_pin: output_idx,
            idx: self.structures.len(),
        });

        self.structures.insert(idx, track);
        self
    }

    pub fn switch(&mut self, switch_type: SwitchType) -> &Self {
        let idx = self.structures.len();

        let input_idx = self.pins.len();
        self.pins.insert(input_idx, idx);

        let output_left_idx = self.pins.len();
        self.pins.insert(output_left_idx, idx);

        let output_right_idx = self.pins.len();
        self.pins.insert(output_right_idx, idx);

        let switch = StructureTypes::Switch(Switch {
            sections: generate_sections(2, false, 50),
            switch_type: switch_type,
            input_pin: input_idx,
            left_output_pin: output_left_idx,
            right_output_pin: output_right_idx,
            idx: idx,
        });

        self.structures.insert(idx, switch);
        self
    }
}
