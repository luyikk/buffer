use crate::serde::error::DataError;
use crate::serde::serialize_impl::{
    DataSerializeMap, DataSerializeSeq, DataSerializeTuple, DataSerializeTupleStruct,
    DataSerializeTupleVariant,
};
use crate::Data;
use anyhow::Result;
use paste::paste;
use serde::{Serialize, Serializer};

macro_rules! make_base_serialize {
    ($($type:ty)+) => {
      paste!{
        $(
        #[inline]
        fn [<serialize_ $type>] (self, v: $type) -> Result<Self::Ok, Self::Error> {
            self.write_fixed(v);
            Ok(())
        }
        )*
      }
    };
}

impl<'a> Serializer for &'a mut Data {
    type Ok = ();
    type Error = DataError;
    type SerializeSeq = DataSerializeSeq<'a>;
    type SerializeTuple = DataSerializeTuple<'a>;
    type SerializeTupleStruct = DataSerializeTupleStruct<'a>;
    type SerializeTupleVariant = DataSerializeTupleVariant<'a>;
    type SerializeMap = DataSerializeMap<'a>;
    type SerializeStruct = DataSerializeTupleStruct<'a>;
    type SerializeStructVariant = DataSerializeTupleVariant<'a>;

    make_base_serialize!(bool i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 i128 u128);

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        self.write_fixed(v.encode_utf8(&mut buf).as_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        self.write_fixed(v);
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        self.write_fixed(v);
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        self.write_fixed(0u8);
        Ok(())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        self.write_fixed(1u8);
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        self.write_fixed(1u8);
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        self.write_fixed(1u8);
        Ok(())
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        self.write_fixed(variant);
        Ok(())
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        self.write_fixed(variant);
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        Ok(DataSerializeSeq::new(self, len))
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        Ok(DataSerializeTuple::new(self, len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        Ok(DataSerializeTupleStruct::new(self, len))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        Ok(DataSerializeTupleVariant::new(self, variant, len))
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        Ok(DataSerializeMap::new(self, len))
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        Ok(DataSerializeTupleStruct::new(self, len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        if self.mode == 1 {
            return Err(DataError::RollBack);
        }
        Ok(DataSerializeTupleVariant::new(self, variant, len))
    }
}
