use serde::{Serializer, Serialize};
use crate::Data;
use std::fmt::Display;
use crate::serde::error::DataError;
use crate::serde::serialize_impl::*;

impl<'a> Serializer for &'a mut Data{
    type Ok = ();
    type Error = DataError;
    type SerializeSeq = DataSerializeSeq<'a>;
    type SerializeTuple = DataSerializeTuple<'a>;
    type SerializeTupleStruct = DataSerializeTupleStruct<'a>;
    type SerializeTupleVariant =DataSerializeTupleVariant<'a>;
    type SerializeMap = DataSerializeMap<'a>;
    type SerializeStruct = DataSerializeTupleStruct<'a>;
    type SerializeStructVariant = DataSerializeTupleVariant<'a>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
       self.write_to_le(&v);
       Ok(())
    }
    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        self.write_to_le(&v.encode_utf8(&mut buf).as_bytes());
        Ok(())
    }
    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&v);
        Ok(())
    }
    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.write_to_le(&0u8);
        Ok(())
    }
    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        self.write_to_le(&1u8);
        value.serialize(self)
    }
    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        self.write_to_le(&1u8);
        Ok(())
    }
    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        self.write_to_le(&1u8);
        Ok(())
    }
    #[inline]
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        self.write_to_le(&variant);
        Ok(())
    }
    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        self.write_to_le(&variant);
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        Ok(DataSerializeSeq::new(self,len))
    }
    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        Ok(DataSerializeTuple::new(self,len))
    }
    #[inline]
    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        Ok(DataSerializeTupleStruct::new(self,len))
    }
    #[inline]
    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        Ok(DataSerializeTupleVariant::new(self,variant,len))
    }
    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        Ok(DataSerializeMap::new(self,len))
    }
    #[inline]
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        Ok(DataSerializeTupleStruct::new(self,len))
    }
    #[inline]
    fn serialize_struct_variant(self,_name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        if self.mode==1{
            return Err(DataError::Str("rollback".to_string()));
        }
        Ok(DataSerializeTupleVariant::new(self,variant,len))
    }
    #[inline]
    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Display {
        self.write_to_le(&format!("{}",value));
        Ok(())
    }
}


impl Data{
    #[inline]
    pub fn serde_serialize<T:Serialize>(&mut self,value:T)->Result<(),DataError>{
        value.serialize(self)
    }
}