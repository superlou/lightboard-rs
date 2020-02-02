use std::collections::HashMap;
use ggez::nalgebra::Point2;
use crate::light::{Color, Intensity};

#[derive(Debug)]
pub enum ElementKind {
    Rgbiu{rgb: Color, uv: Intensity},
    Rgbi(Color),
    Uv(Intensity),
    Smoke,
    Actuator,
    Gobo,
    Unknown,
}

#[derive(Debug)]
pub struct Element {
    kind: ElementKind,
    channels: HashMap<String, u8>,
}

impl Element {
    pub fn new(kind: ElementKind) -> Element {
        Element { kind, channels: HashMap::new() }
    }

    pub fn add_channel(&mut self, name: &str, index: u8) {
        self.channels.insert(name.to_owned(), index);
    }

    pub fn kind(&self) -> &ElementKind {
        &self.kind
    }

    pub fn set_kind(&mut self, kind: ElementKind) {
        self.kind = kind;
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
            match &element.kind {
                ElementKind::Rgbi(color) => {
                    if let Some(channel) = element.channels.get("i") {
                        self.dmx_vec[(channel - 1) as usize] = 255;
                    }

                    if let Some(channel) = element.channels.get("r") {
                        self.dmx_vec[(channel - 1) as usize] = (color.r() * 255.0) as u8;
                    }

                    if let Some(channel) = element.channels.get("g") {
                        self.dmx_vec[(channel - 1) as usize] = (color.g() * 255.0) as u8;
                    }

                    if let Some(channel) = element.channels.get("b") {
                        self.dmx_vec[(channel - 1) as usize] = (color.b() * 255.0) as u8;
                    }
                },
                ElementKind::Rgbiu{rgb: color, uv: uv} => {
                    if let Some(channel) = element.channels.get("i") {
                        self.dmx_vec[(channel - 1) as usize] = 255;
                    }

                    if let Some(channel) = element.channels.get("r") {
                        self.dmx_vec[(channel - 1) as usize] = (color.r() * 255.0) as u8;
                    }

                    if let Some(channel) = element.channels.get("g") {
                        self.dmx_vec[(channel - 1) as usize] = (color.g() * 255.0) as u8;
                    }

                    if let Some(channel) = element.channels.get("b") {
                        self.dmx_vec[(channel - 1) as usize] = (color.b() * 255.0) as u8;
                    }

                    if let Some(channel) = element.channels.get("uv") {
                        self.dmx_vec[(channel - 1) as usize] = (uv * 255.0) as u8;
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
