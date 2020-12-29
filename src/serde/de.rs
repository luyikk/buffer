use serde::{Deserializer, Deserialize};
use crate::Data;
use serde::de::{Visitor, SeqAccess, DeserializeSeed, EnumAccess};
use crate::serde::error::DataError;
use paste::paste;
use serde::export::PhantomData;

impl Data{
    pub fn serde_deserialize<'a,T:Deserialize<'a>>(&'a mut self)->Result<T,DataError>{
        T::deserialize(self)
    }
}

macro_rules! make_deserialize {
    ($t:ty) => {
        paste!{
             fn [<deserialize_ $t>]<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                let value= visitor.[<visit_ $t>]( self.get_le::<$t>()?);
                match value {
                    Ok(value)=>Ok(value),
                    Err(err)=>Err(err)
                }
             }
         }
    };
}

impl<'de,'a> Deserializer<'de> for &'a mut Data{
    type Error = DataError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_byte_buf(self.to_vec())
    }

    make_deserialize!(bool);
    make_deserialize!(i8);
    make_deserialize!(u8);
    make_deserialize!(i16);
    make_deserialize!(u16);
    make_deserialize!(i32);
    make_deserialize!(u32);
    make_deserialize!(i64);
    make_deserialize!(u64);
    make_deserialize!(f32);
    make_deserialize!(f64);


    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let len= self.get_le::<u32>()? as usize;
        if self.offset+len >self.len(){
            return Err(DataError::Str("deserialize_bytes:offset + len > max len".into()))
        }
        else {
            let bak = self.offset;
            self.offset += len;
            let str=String::from_utf8_lossy(&self[bak..self.offset]);
            let value = visitor.visit_str (&str);
            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(err)
            }
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let len= self.get_le::<u32>()? as usize;
        if self.offset+len >self.len(){
            return Err(DataError::Str("deserialize_bytes:offset + len > max len".into()))
        }
        else {
            let bak = self.offset;
            self.offset += len;
            let value = visitor.visit_bytes(&self[bak..self.offset]);
            match value {
                Ok(value) => Ok(value),
                Err(err) => Err(err)
            }
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if self.get_le::<u8>()?==0{
            visitor.visit_none()
        }
        else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if self.get_le::<u8>()?==1{
            visitor.visit_unit()
        }
        else{
            Err(DataError::Str("deserialize_unit: current data !=0u8".into()))
        }
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let value=visitor.visit_seq(SeqAssess::new(self)?);
        match value {
            Ok(value)=>Ok(value),
            Err(err)=>Err(err)
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let value=visitor.visit_map(MapAccess::new(self)?);
        match value {
            Ok(value)=>Ok(value),
            Err(err)=>Err(err)
        }
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        let value=visitor.visit_map(MapAccess::new(self)?);
        match value {
            Ok(value)=>Ok(value),
            Err(err)=>Err(err)
        }
    }

    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {

        let value=visitor.visit_enum(VariantAccess::new(self)?);
        match value {
            Ok(value)=>Ok(value),
            Err(err)=>Err(err)
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }
}

struct SeqAssess<'a>{
    len:u32,
    current:u32,
    data:&'a mut Data,
}

impl <'a> SeqAssess<'a>{
    pub fn new(data:&'a mut Data)->Result<Self,DataError>{
        let len=data.serde_deserialize::<u32>()?;
        Ok(SeqAssess{
            len,
            current:0,
            data
        })
    }
}

impl<'a,'de> serde::de::SeqAccess<'de> for SeqAssess<'a>{
    type Error = DataError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where
        T: DeserializeSeed<'de> {

        if self.current<self.len {
            let r = seed.deserialize(&mut *self.data)?;
            self.current+=1;
            Ok(Some(r))
        }
        else{
            Ok(None)
        }

    }
}

struct MapAccess<'a>{
    data:&'a mut Data,
    len:u32,
    current:u32
}

impl <'a> MapAccess<'a>{
    pub fn new(data:&'a mut Data)->Result<Self,DataError>{
        let len=data.get_le::<u32>()?;
        Ok(MapAccess{
            data,
            len,
            current:0
        })
    }
}

impl <'de,'a> serde::de::MapAccess<'de> for MapAccess<'a>{
    type Error = DataError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, Self::Error> where
        K: DeserializeSeed<'de> {
        if self.current<self.len{
            let r= seed.deserialize(&mut *self.data)?;
            self.current+=1;
            Ok(Some(r))
        }
        else{
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<<V as DeserializeSeed<'de>>::Value, Self::Error> where
        V: DeserializeSeed<'de> {
        Ok(seed.deserialize(&mut *self.data)?)
    }
}

struct VariantAccess<'a>{
    data:&'a mut Data
}

impl <'a> VariantAccess<'a>{
    pub fn new(data:&'a mut Data)->Result<Self,DataError>{
        Ok(VariantAccess{
            data
        })
    }
}

impl <'de,'a> serde::de::EnumAccess<'de> for VariantAccess<'a>{
    type Error = DataError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(<V as DeserializeSeed<'de>>::Value, Self::Variant), Self::Error> where
        V: DeserializeSeed<'de> {
        Ok((seed.deserialize(&mut *self.data)?,self))
    }
}


impl <'de,'a> serde::de::VariantAccess<'de> for VariantAccess<'a>{
    type Error = DataError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        serde::de::Deserialize::deserialize(self.data)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as DeserializeSeed<'de>>::Value, Self::Error> where
        T: DeserializeSeed<'de> {
        seed.deserialize(self.data)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        serde::de::Deserializer::deserialize_seq(self.data, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        serde::de::Deserializer::deserialize_struct(self.data, "", fields, visitor)
    }
}