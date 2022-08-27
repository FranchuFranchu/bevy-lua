use bevy_reflect::*;
use rlua::prelude::LuaValue;

use crate::{to_lua::to_lua, from_lua::from_lua};

#[derive(Debug, Reflect, FromReflect)]
pub struct A {
    q: (i8, f32, String),
}

#[derive(Debug, Reflect, FromReflect)]
pub struct B {
    q: (i8, f32, String),
    z: bool
}

#[test]
fn main() {
    let l = rlua::Lua::new();
    let mut reg = TypeRegistry::new();
    reg.register::<Vec<usize>>();
    reg.register::<String>();
    reg.register::<(i8, f32, String)>();
    
    l.context(|cx| {
        
        let b = B {
            q: (2, 34.1, String::from("Hasdasd")),
            z: false,
        };
        let y = to_lua(cx, b.as_reflect());
        cx.globals().set("outside", y).unwrap();
        cx.load("for k, v in pairs(outside) do print(k, v) end").exec().unwrap();
        let v = cx.load(r#"{q = {1, 3.1, "wiwi wowo"}}"#).eval::<LuaValue>().unwrap();
        let q = from_lua(&reg, A::type_info(), v);
        
        
        
        println!("{:?}", A::from_reflect(&*q));
    });
    
    println!("Hello, world!");
}
