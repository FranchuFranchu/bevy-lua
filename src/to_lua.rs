use rlua::prelude::*;
use bevy_reflect::{Reflect, Struct, Tuple, TupleStruct};

fn value_to_lua<'cx>(cx: LuaContext<'cx>, data: &dyn Reflect) -> LuaValue<'cx> {
	macro_rules! downcast_integer {
	    ($type:ident) => {       
			if let Some(data) = data.downcast_ref::<$type>() {
				return LuaValue::Integer(*data as i64)
			}
	    };
	}
	macro_rules! downcast_float {
	    ($type:ident) => {       
			if let Some(data) = data.downcast_ref::<$type>() {
				return LuaValue::Number(*data as f64)
			}
	    };
	}
	downcast_integer!(usize);
	downcast_integer!(u8);
	downcast_integer!(u16);
	downcast_integer!(u32);
	downcast_integer!(u64);
	downcast_integer!(i8);
	downcast_integer!(i16);
	downcast_integer!(i32);
	downcast_integer!(i64);
	downcast_float!(f32);
	downcast_float!(f64);
	if let Some(data) = data.downcast_ref::<bool>() {
		return LuaValue::Boolean(*data)
	}
	if let Some(data) = data.downcast_ref::<String>() {
		return LuaValue::String(cx.create_string(data).unwrap())
	}
	
	todo!()
}

fn struct_to_lua<'cx>(cx: LuaContext<'cx>, data: &dyn Struct) -> LuaTable<'cx> {
	let table = cx.create_table().unwrap();
	for i in 0..data.field_len() {
		let key = data.name_at(i).unwrap();
		let value = data.field_at(i).unwrap();
		table.set(key, to_lua(cx, value)).unwrap();
	}
	table
}

fn tuple_to_lua<'cx>(cx: LuaContext<'cx>, data: &dyn Tuple) -> LuaTable<'cx> {
	let table = cx.create_table().unwrap();
	for i in 0..data.field_len() {
		let value = data.field(i).unwrap();
		table.set(i, to_lua(cx, value)).unwrap();
	}
	table
}

fn tuple_struct_to_lua<'cx>(cx: LuaContext<'cx>, data: &dyn TupleStruct) -> LuaTable<'cx> {
	let table = cx.create_table().unwrap();
	for i in 0..data.field_len() {
		let value = data.field(i).unwrap();
		table.set(i, to_lua(cx, value)).unwrap();
	}
	table
}

pub fn to_lua<'cx>(cx: LuaContext<'cx>, data: &dyn Reflect) -> LuaValue<'cx> {
	match data.reflect_ref() {
	    bevy_reflect::ReflectRef::Struct(data) => LuaValue::Table(struct_to_lua(cx, data)),
	    bevy_reflect::ReflectRef::TupleStruct(data) => LuaValue::Table(tuple_struct_to_lua(cx, data)),
	    bevy_reflect::ReflectRef::Tuple(data) => LuaValue::Table(tuple_to_lua(cx, data)),
	    bevy_reflect::ReflectRef::List(_) => todo!(),
	    bevy_reflect::ReflectRef::Array(_) => todo!(),
	    bevy_reflect::ReflectRef::Map(_) => todo!(),
	    bevy_reflect::ReflectRef::Value(data) => value_to_lua(cx, data),
	}
}