use crate::Data;
use serde::{Serialize, Deserialize};
use std::any::{TypeId, Any};
use anyhow::*;

impl Data{
    pub fn msgpack_to<'a,T:Deserialize<'a>>(&'a mut self) ->Result<T>{
        Ok(rmp_serde::decode::from_read_ref(&self[..])?)
    }

    pub fn msgpack_from<T: Serialize>(value:T) ->Result<Data> {
        Ok(rmp_serde::encode::to_vec(&value)?.into())
    }

    pub fn msgpack_deserialize<'a,T:Deserialize<'a>+'static>(&'a mut self)->Result<T> {
        let typeid = TypeId::of::<T>();

        if typeid == TypeId::of::<bool>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<i8>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<u8>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<i16>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<u16>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<i32>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<u32>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<i64>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<u64>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<f32>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<f32>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<Vec<u8>>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<&[u8]>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<&str>() {
            Ok(self.serde_deserialize()?)
        } else if typeid == TypeId::of::<String>() {
            Ok(self.serde_deserialize()?)
        } else {
            let len = self.get_le::<u32>()? as usize;
            let start = self.offset;
            if !self.set_position(start + len) {
                bail!("index overflow {}", line!())
            }
            Ok( rmp_serde::decode::from_read_ref(&self[start..self.offset])?)
        }
    }

    pub fn msgpack_serialize<T:Serialize+'static>(&mut self,value:T)->Result<()>{
        let typeid =value.type_id();
        if typeid == TypeId::of::<bool>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<i8>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<u8>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<i16>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<u16>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<i32>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<u32>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<i64>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<u64>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<f32>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<f32>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<Vec<u8>>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<&[u8]>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<&str>() {
            self.serde_serialize(value)?;
        } else if typeid == TypeId::of::<String>() {
            self.serde_serialize(value)?;
        } else {
            let buff = rmp_serde::encode::to_vec(&value)?;
            self.serde_serialize(buff)?;
        }

        Ok(())
    }
}

