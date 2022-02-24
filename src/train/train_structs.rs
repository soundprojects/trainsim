// use std::collections::HashMap;
// use std::hash::Hash;

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
// #[derive(Debug, Clone)]
// pub struct Switch {
//     pub sections: Vec<Section>,
//     pub switch_type: SwitchType,
// }

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
//#[derive(Debug, Copy, Clone)]
// pub enum SwitchType {
//     LeftSplitUp,
//     LeftSplitDown,
//     RightSplitUp,
//     RightSplitDown,
// }

pub trait Identifiable {
    fn get_id() {}
}

// #[derive(Debug, Clone)]
// pub struct TrainSystem<K: Identifiable, V: Identifiable> {
//     pub connections: HashMap<i32, i32>,
//     pub structures: HashMap<K, V>,
// }

// impl<K, V> TrainSystem<K, V>
// where
//     K: Identifiable + Eq + Hash,
//     V: Identifiable,
// {
//     fn new(&mut self) -> &Self {
//         self.connections = HashMap::new();
//         self.structures = HashMap::new();
//         self
//     }

//     fn connect(&mut self, key: K, value: V) {
//         self.structures.insert(key, value);
//     }
// }
