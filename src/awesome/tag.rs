//! TODO Fill in

use std::fmt::{self, Display, Formatter};
use std::default::Default;
use rlua::{self, Table, Lua, UserData, ToLua, Value, UserDataMethods};
use super::object::{self, Object, Objectable};
use super::class::{self, Class, ClassBuilder};

#[derive(Clone, Debug)]
pub struct TagState {
    // TODO Fill in
    dummy: i32
}

pub struct Tag<'lua>(Object<'lua>);

impl Default for TagState {
    fn default() -> Self {
        TagState {
            dummy: 0
        }
    }
}

impl <'lua> Tag<'lua> {
    fn new(lua: &'lua Lua, args: Table) -> rlua::Result<Object<'lua>> {
        let class = class::class_setup(lua, "tag")?;
        Ok(Tag::allocate(lua, class)?
           .handle_constructor_argument(args)?
           .build())
    }
}

impl Display for TagState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Tag: {:p}", self)
    }
}

impl <'lua> ToLua<'lua> for Tag<'lua> {
    fn to_lua(self, lua: &'lua Lua) -> rlua::Result<Value<'lua>> {
        self.0.to_lua(lua)
    }
}

impl UserData for TagState {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        object::default_add_methods(methods);
    }
}

pub fn init(lua: &Lua) -> rlua::Result<Class> {
    method_setup(lua, Class::builder(lua, "tag", None)?)?
        .save_class("tag")?
        .build()
}

fn method_setup<'lua>(lua: &'lua Lua, builder: ClassBuilder<'lua>) -> rlua::Result<ClassBuilder<'lua>> {
    // TODO Do properly
    use super::dummy;
    builder.method("connect_signal".into(), lua.create_function(dummy)?)?
           .method("__call".into(), lua.create_function(|lua, args: Table| Tag::new(lua, args))?)
}

impl_objectable!(Tag, TagState);
