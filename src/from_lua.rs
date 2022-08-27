use bevy_reflect::{DynamicStruct, TypeInfo, StructInfo, TypeRegistry, ValueInfo, Reflect, DynamicList, ListInfo, DynamicTuple, TupleInfo, DynamicTupleStruct, TupleStructInfo};
use rlua::prelude::*;

pub fn table_to_struct(reg: &TypeRegistry, info: &StructInfo, table: LuaTable) -> DynamicStruct {
	let mut remaining_fields = info.field_len();
	let mut s = DynamicStruct::default();
	for r in table.pairs::<String, LuaValue>() {
		if remaining_fields == 0 {
			todo!();
		}
		remaining_fields -= 1;
		let (k, lua_value) = r.unwrap();
		let value_type_id = info.field(&k).unwrap().type_id();
		s.insert_boxed(&k, from_lua(reg, reg.get_type_info(value_type_id).unwrap(), lua_value))
	}
	s
}

pub fn value_to_value(reg: &TypeRegistry, info: &ValueInfo, value: LuaValue) -> Box<dyn Reflect> {
	macro_rules! number_to_value {
	    ($type:ident) => {
	        if info.is::<$type>() {
				return match value {
					LuaValue::Number(value) => {
						(value as $type).as_reflect().clone_value()
					}
					LuaValue::Integer(value) => {
						(value as $type).as_reflect().clone_value()
					}
					_ => todo!("Can't convert Lua value {:?} Rust type {}", value, info.type_name())
				}
			}
	    };
	}
	macro_rules! number_to_value_bool {
	    ($type:ident) => {
	        if info.is::<$type>() {
				return match value {
					LuaValue::Boolean(value) => {
						(value as $type).as_reflect().clone_value()
					}
					LuaValue::Number(value) => {
						(value as $type).as_reflect().clone_value()
					}
					LuaValue::Integer(value) => {
						(value as $type).as_reflect().clone_value()
					}
					_ => todo!("Can't convert Lua value {:?} Rust type {}", value, info.type_name())
				}
			}
	    };
	}
	number_to_value_bool!(usize);
	number_to_value_bool!(u8);
	number_to_value_bool!(u16);
	number_to_value_bool!(u32);
	number_to_value_bool!(u64);
	number_to_value_bool!(i8);
	number_to_value_bool!(i16);
	number_to_value_bool!(i32);
	number_to_value_bool!(i64);
	number_to_value!(f32);
	number_to_value!(f64);
	
    if info.is::<bool>() {
		return match value {
			LuaValue::Boolean(value) => {
				value.as_reflect().clone_value()
			}
			_ => todo!("Can't convert Lua value {:?} Rust type {}", value, info.type_name())
		}
	}
	
    if info.is::<String>() {
		return match value {
			LuaValue::String(value) => {
				value.to_str().unwrap().to_string().as_reflect().clone_value()
			}
			_ => todo!("Can't convert Lua value {:?} Rust type {}", value, info.type_name())
		}
	}
	todo!("Can't convert Lua value {:?} Rust type {}", value, info.type_name())
}

pub fn value_is_nil(value: &LuaValue) -> bool {
	if let LuaValue::Nil = value {
		true
	} else {
		false
	}
}

pub fn table_to_vec(reg: &TypeRegistry, item_type: &TypeInfo, table: LuaTable) -> Vec<Box<dyn Reflect>> {
	let mut s = Vec::new();
	let mut idx = 1;
	loop {
		let value: LuaValue = table.get(idx).unwrap();
		if value_is_nil(&value) {
			break;
		}
		s.push(from_lua(reg, &item_type, value));
		idx += 1;
	}
	s
}

pub fn table_to_list(reg: &TypeRegistry, info: &ListInfo, table: LuaTable) -> DynamicList {
	let mut l = DynamicList::default();
	for i in table_to_vec(reg, reg.get_type_info(info.item_type_id()).unwrap(), table).into_iter() {
		l.push_box(i);
	}
	l
}
pub fn table_to_tuple(reg: &TypeRegistry, info: &TupleInfo, table: LuaTable) -> DynamicTuple {
	let mut s = DynamicTuple::default();
	let mut idx = 0;
	loop {
		let value: LuaValue = table.get(idx + 1).unwrap();
		if value_is_nil(&value) {
			break;
		}
		s.insert_boxed(from_lua(reg, reg.get_type_info(info.field_at(idx).unwrap().type_id()).unwrap(), value));
		idx += 1;
	}
	s
}
pub fn table_to_tuple_struct(reg: &TypeRegistry, info: &TupleStructInfo, table: LuaTable) -> DynamicTupleStruct {
	let mut s = DynamicTupleStruct::default();
	let mut idx = 0;
	loop {
		let value: LuaValue = table.get(idx + 1).unwrap();
		if value_is_nil(&value) {
			break;
		}
		s.insert_boxed(from_lua(reg, reg.get_type_info(info.field_at(idx).unwrap().type_id()).unwrap(), value));
		idx += 1;
	}
	s
}


pub fn from_lua(reg: &TypeRegistry, type_info: &TypeInfo, value: LuaValue) -> Box<dyn Reflect> {
	match type_info {
		TypeInfo::Struct(info) => {
			match value {
				LuaValue::Table(table) => table_to_struct(reg, info, table).as_reflect().clone_value(),
				_ => todo!()
			}
		}
		TypeInfo::List(info) => {
			match value {
				LuaValue::Table(table) => table_to_list(reg, info, table).as_reflect().clone_value(),
				_ => todo!()
			}
		}
		TypeInfo::TupleStruct(info) => {
			match value {
				LuaValue::Table(table) => table_to_tuple_struct(reg, info, table).as_reflect().clone_value(),
				_ => todo!()
			}
		}
		TypeInfo::Tuple(info) => {
			match value {
				LuaValue::Table(table) => table_to_tuple(reg, info, table).as_reflect().clone_value(),
				_ => todo!()
			}
		}
		TypeInfo::Dynamic(info) => {
			todo!();
		}
		TypeInfo::Value(info) => {
			value_to_value(reg, info, value)
		},
		_ => todo!(),
	}
}