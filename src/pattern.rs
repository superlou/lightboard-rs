use std::fs::read_to_string;
use std::fmt;
use std::collections::HashMap;
use rlua::{Lua, Function, Table, ToLua, Context};

pub struct Pattern {
    lua: Option<Lua>,
    group: String,
    property: String,
    script_name: String,
    options: HashMap<String, toml::Value>,
    element_count: usize,
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pattern")
            .field("group", &self.group)
            .field("property", &self.property)
            .field("script_name", &self.script_name)
            .finish()
    }
}

#[derive(Debug)]
struct PatternOption {
    default: u32,
    desc: String,
}

fn toml_to_lua<'a>(toml_value: &toml::Value, ctx: Context<'a>) -> rlua::Result<rlua::Value<'a>> {
    match toml_value {
        toml::Value::String(s) => s.to_owned().to_lua(ctx),
        toml::Value::Integer(x) => x.to_lua(ctx),
        toml::Value::Float(x) => x.to_lua(ctx),
        toml::Value::Boolean(b) => b.to_lua(ctx),
        toml::Value::Datetime(dt) => dt.to_string().to_lua(ctx),
        _ => Err(rlua::Error::ToLuaConversionError{
            from: "Unexpected TOML type",
            to: "",
            message: None,
        }),
    }
}

impl Pattern {
    pub fn new(script_name: &str, group: &str, property: &str, element_count: usize,
               options: HashMap<String, toml::Value>) -> Self
    {
        let script = "patterns/".to_owned() + script_name;
        let script = read_to_string(script).unwrap();
        let lua = match Pattern::build_lua_env(&script, group, element_count, &options) {
            Ok(lua) => Some(lua),
            Err(e) => {
                dbg!(e);
                None
            },
        };

        Self {
            lua,
            group: group.to_owned(),
            property: property.to_owned(),
            script_name: script_name.to_owned(),
            options,
            element_count,
        }
    }

    pub fn reload(&mut self) {
        let script = "patterns/".to_owned() + &self.script_name;
        let script = read_to_string(script).unwrap();

        let group = &self.group;
        let element_count = self.element_count;
        let options = &self.options;

        self.lua = match Pattern::build_lua_env(&script, group, element_count, options) {
            Ok(lua) => Some(lua),
            Err(e) => {
                dbg!(e);
                None
            },
        };
    }

    fn build_lua_env(script: &str, group: &str, element_count: usize,
                     options: &HashMap<String, toml::Value>) -> Result<Lua, rlua::Error>
    {
        let lua = Lua::new();

        lua.context(|ctx| {
            let globals = ctx.globals();

            globals.set("group_name", group.to_owned())?;
            globals.set("element_count", element_count)?;
            let options_table = ctx.create_table()?;
            globals.set("options", options_table)?;

            ctx.load(&script).exec()?;

            let setup: Function = globals.get("setup")?;

            setup.call::<(), ()>(())?;

            let options_table: Table = globals.get("options")?;

            for pair in options_table.pairs::<String, Table>() {
                let (name, option_table) = pair?;

                if let Some(value) = options.get(&name) {
                    option_table.set("default", toml_to_lua(value, ctx)?)?;
                }

                let default: rlua::Value = option_table.get("default")?;
                option_table.set("value", default)?;
            }

            ctx.load("").eval::<()>() // todo Why can't I use Ok(())?
        })?;

        Ok(lua)
    }

    pub fn update(&mut self) -> Vec<i32> {
        let mut values = vec![];
        let dt = 1.0 / 30.0;

        match &self.lua {
            Some(lua) => {
                lua.context(|ctx| {
                    let globals = ctx.globals();
                    let update: Function = globals.get("update").unwrap();

                    values = match update.call::<f32, Vec<i32>>(dt) {
                        Ok(x) => x,
                        Err(e) => {
                            println!("Lua error in {}:update", self.script_name);
                            println!("{}", e);
                            vec![]
                        }
                    };

                    ctx.load("").eval::<()>() // todo Why can't I use Ok(())?
                }).unwrap();
            },
            None => {},
        }

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
        let mut options: HashMap<String, toml::Value> = HashMap::new();
        options.insert("color".to_owned(), 0x123456.into());
        let mut p = Pattern::new("constant.lua", "group1", "color", 2, options);
        assert_eq!([0x123456, 0x123456], p.update().as_slice());
    }
}
