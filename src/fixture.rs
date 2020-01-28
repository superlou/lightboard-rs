use std::collections::HashMap;
use ggez::nalgebra::Point2;

#[derive(Debug)]
pub enum ElementType {
    Rgbiu,
    Rgbi,
    Uv,
    Smoke,
    Actuator,
    Gobo,
    Unknown,
}

#[derive(Debug)]
pub struct Element {
    kind: ElementType,
    color: (f32, f32, f32),
    intensity: f32,
    channels: HashMap<String, u8>,
}

impl Element {
    pub fn new(kind: ElementType, color: (f32, f32, f32), intensity: f32) -> Element {
        Element { kind, color, intensity, channels: HashMap::new() }
    }

    pub fn add_channel(&mut self, name: &str, index: u8) {
        self.channels.insert(name.to_owned(), index);
    }

    pub fn kind(&self) -> &ElementType {
        &self.kind
    }

    pub fn color(&self) -> (f32, f32, f32) {
        self.color
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = (r, g, b);
    }

    pub fn set_intensity(&mut self, i: f32) {
        self.intensity = i;
    }
}

#[derive(Debug)]
pub struct Fixture {
    pub elements: HashMap<String, Element>,
    pub pos: Point2<f32>,
    pub dmx_vec: Vec<u8>,
    pub channel: usize,
}

impl Fixture {
    pub fn new(pos: Point2<f32>, channel: usize, num_channels: usize) -> Self {
        let mut dmx_vec = vec![];
        dmx_vec.resize(num_channels, 0);

        Self {
            elements: HashMap::new(),
            pos: pos,
            dmx_vec: dmx_vec,
            channel: channel,
        }
    }

    pub fn update_dmx(&mut self) {
        // This only supports basic RGB fixtures right now
        for (_name, element) in self.elements.iter() {
            match element.kind {
                ElementType::Rgbiu | ElementType::Rgbiu => {
                    if let Some(channel) = element.channels.get("i") {
                        self.dmx_vec[(channel - 1) as usize] = 255;
                    }

                    if let Some(channel) = element.channels.get("r") {
                        self.dmx_vec[(channel - 1) as usize] = (element.color.0 * 255.0) as u8;
                    }

                    if let Some(channel) = element.channels.get("g") {
                        self.dmx_vec[(channel - 1) as usize] = (element.color.1 * 255.0) as u8;
                    }

                    if let Some(channel) = element.channels.get("b") {
                        self.dmx_vec[(channel - 1) as usize] = (element.color.2 * 255.0) as u8;
                    }
                },
                _ => {}
            }
        }
    }

    pub fn dmx(&self) -> &Vec<u8> {
        &self.dmx_vec
    }
}
