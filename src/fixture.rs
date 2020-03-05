use std::collections::HashMap;
use ggez::nalgebra::Point2;
use crate::light::{Color, Intensity};

#[derive(Debug)]
pub enum ElementKind {
    Intensity(Intensity),
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
    pos: (usize, usize),
}

impl Element {
    pub fn new(kind: ElementKind) -> Element {
        Element { kind, channels: HashMap::new(), pos: (0, 0) }
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

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.pos = (x, y);
    }

    pub fn arrange_linear(elements: &mut HashMap<String, Element>) {
        let mut element_names: Vec<String> = elements.iter().map(|(name, _element)| name.clone()).collect();
        element_names.sort();
        for (i, name) in element_names.iter().enumerate() {
            elements.get_mut(name).unwrap().set_pos(i, 0);
        }
    }
}

#[derive(Debug)]
pub struct Fixture {
    elements: HashMap<String, Element>,
    pos: Point2<f32>,
    dmx_vec: Vec<u8>,
    channel: usize,
}

impl Fixture {
    pub fn new(mut elements: HashMap<String, Element>, pos: Point2<f32>,
               channel: usize, num_channels: usize) -> Self
    {
        let mut dmx_vec = vec![];
        dmx_vec.resize(num_channels, 0);

        Element::arrange_linear(&mut elements);

        Self {
            elements,
            pos,
            dmx_vec,
            channel,
        }
    }

    pub fn channel(&self) -> usize {
        self.channel
    }

    pub fn pos(&self) -> Point2<f32> {
        self.pos
    }

    pub fn elements(&self) -> &HashMap<String, Element> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut HashMap<String, Element> {
        &mut self.elements
    }

    pub fn update_dmx(&mut self) {
        // This only supports basic fixture elements right now
        for (_name, element) in self.elements.iter() {
            match &element.kind {
                ElementKind::Intensity(intensity) => {
                    if let Some(channel) = element.channels.get("i") {
                        self.dmx_vec[(channel - 1) as usize] = (intensity * 255.0) as u8;
                    }
                },
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
                ElementKind::Rgbiu{rgb: color, uv} => {
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
