use colorsys::Rgb;
use eframe::egui::Color32;

///This Utility method allows us to pass a Hex formatted color string to Egui
///E.g. "#000000" will be converted to Rgb(255,255,255)
pub trait ColorHex: Sized {
    fn to_hex(&self) -> String;
    fn from_hex(hex: &str) -> Option<Self>;

    fn from_hex_panic(hex: &str) -> Self {
        Self::from_hex(hex).unwrap_or_else(|| panic!("Failed to parse color: {}", hex))
    }
}

impl ColorHex for Color32 {
    fn to_hex(&self) -> String {
        let color = Rgb::new(
            self.r().into(),
            self.g().into(),
            self.b().into(),
            Some(self.a().into()),
        );
        color.to_hex_string()
    }

    fn from_hex(hex: &str) -> Option<Self> {
        if let Ok(color) = Rgb::from_hex_str(hex) {
            let color =
                Color32::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8);
            return Some(color);
        }

        None
    }
}
