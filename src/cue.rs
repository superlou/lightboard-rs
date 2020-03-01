use std::result::Result;
use crate::show_loader::build_cue_list_from_config;

pub struct Cue {
    name: String,
    command: String,
}

pub struct CueList {
    cues: Vec<Cue>
}

impl CueList {
    pub fn new() -> CueList {
        CueList { cues: vec![] }
    }

    pub fn new_from_config(config_file: &str) -> CueList {
        build_cue_list_from_config(config_file)
    }

    pub fn add(&mut self, name: &str, command: &str) {
        let cue = Cue {
            name: name.to_owned(),
            command: command.to_owned(),
        };

        self.cues.push(cue);
    }

    pub fn cues(&self) -> &Vec<Cue> {
        &self.cues
    }

    pub fn cue_command(&self, index: usize) -> Result<String, ()> {
        if index < self.cues.len() {
            Ok(self.cues[index].command.to_owned())
        } else {
            Err(())
        }
    }
}

impl Cue {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn command(&self) -> &str {
        &self.command
    }
}
