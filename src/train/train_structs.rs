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
}

///Switch
///Struct to hold a simple switch
#[derive(Debug, Clone)]
pub struct Switch {
    pub sections: Vec<Section>,
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
