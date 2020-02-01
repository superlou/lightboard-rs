use std::fs::read_to_string;
use std::fmt;
use rlua::{Lua, Function, Table};

pub struct Pattern {
    lua: Lua,
    group: String,
    options: Vec<PatternOption>
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pattern")
            .field("group", &self.group)
            .field("options", &self.options)
            .finish()
    }
}

#[derive(Debug)]
struct PatternOption {
    default: u32,
    desc: String,
}

impl Pattern {
    pub fn new(script_name: &str, group: &str, element_count: usize) -> Self {
        let script = "patterns/".to_owned() + script_name;
        let script = read_to_string(script).unwrap();

        let lua = Lua::new();

        let mut options = vec![];

        lua.context(|ctx| {
            let globals = ctx.globals();

            globals.set("group_name", group.to_owned()).unwrap();
            globals.set("element_count", element_count).unwrap();

            ctx.load(&script).exec().unwrap();
            let setup: Function = globals.get("setup").unwrap();

            let options_table = setup.call::<String, Table>(group.to_owned()).unwrap();

            for pair in options_table.pairs::<String, Table>() {
                let (name, option_table) = pair.unwrap();
                let default: u32 = option_table.get("default").unwrap();
                let desc: String = option_table.get("desc").unwrap();

                options.push(PatternOption {default, desc});
            }

            ctx.load("").eval::<()>() // todo Why can't I use Ok(())?
        }).unwrap();

        Self {
            lua: lua,
            group: group.to_owned(),
            options: options,
        }
    }

    pub fn update(&mut self) -> Vec<i32> {
        let mut values = vec![];

        self.lua.context(|ctx| {
            let globals = ctx.globals();
            let update: Function = globals.get("update").unwrap();
            values = update.call::<(), Vec<i32>>(()).unwrap();

            ctx.load("").eval::<()>() // todo Why can't I use Ok(())?
        }).unwrap();

        values
    }

    pub fn group(&self) -> &str {
        &self.group
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_creation() {
        let mut p = Pattern::new("constant.lua", "group1", 2);
        assert_eq!([0x123456, 0x123456], p.update().as_slice());
    }
}
