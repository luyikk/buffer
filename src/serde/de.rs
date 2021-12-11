use crate::serde::error::DataError;
use crate::DataReader;
use anyhow::{anyhow, Result};
use paste::paste;
use serde::de::{DeserializeSeed, Visitor};
use serde::Deserializer;

macro_rules! make_deserialize {
    ($t:ty) => {
         paste!{
             #[inline]
             fn [<deserialize_ $t>]<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
                V: Visitor<'de> {
                visitor.[<visit_ $t>]( self.read_fixed::<$t>()?)
             }
         }
    };
}

impl<'de, 'a, 'b> Deserializer<'de> for &'a mut DataReader<'b>
where
    'b: 'de,
{
    type Error = DataError;

    #[inline]
    fn deserialize_any<V>(self, _visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(DataError::AnyNotSupported)
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

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }
        let str = self.read_fixed_str()?;
        visitor.visit_borrowed_str(str)
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }

        let buff = self.read_fixed_buf()?;
        visitor.visit_borrowed_bytes(buff)
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }

        if self.read_fixed::<u8>()? == 0 {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    #[inline]
    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }

        if self.read_fixed::<u8>()? == 0 {
            return Err(anyhow!("deserialize_unit: current data !=0u8").into());
        }

        visitor.visit_unit()
    }

    #[inline]
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }
        self.deserialize_unit(visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }
        visitor.visit_seq(SeqAssess::new(self)?)
    }

    #[inline]
    fn deserialize_tuple<V>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }
        visitor.visit_map(MapAccess::new(self)?)
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }
        visitor.visit_map(MapAccess::new(self)?)
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }
        visitor.visit_enum(VariantAccess::new(self)?)
    }

    #[inline]
    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.mode == 1 {
            return Err(DataError::Reset);
        }
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_ignored_any<V>(
        self,
        _visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
         Err(DataError::Reset)
    }
}

struct SeqAssess<'a, 'b> {
    len: u32,
    current: u32,
    data: &'a mut DataReader<'b>,
}

impl<'a, 'b> SeqAssess<'a, 'b> {
    #[inline]
    pub fn new(data: &'a mut DataReader<'b>) -> Result<Self, DataError> {
        if data.mode == 1 {
            return Err(DataError::Reset);
        }

        let len = data.read_fixed::<u32>()?;
        Ok(SeqAssess {
            len,
            current: 0,
            data,
        })
    }
}

impl<'a, 'de, 'b: 'de> serde::de::SeqAccess<'de> for SeqAssess<'a, 'b> {
    type Error = DataError;
    #[inline]
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.current < self.len {
            let r = seed.deserialize(&mut *self.data)?;
            self.current += 1;
            Ok(Some(r))
        } else {
            Ok(None)
        }
    }
}

struct MapAccess<'a, 'b> {
    data: &'a mut DataReader<'b>,
    len: u32,
    current: u32,
}

impl<'a, 'b> MapAccess<'a, 'b> {
    #[inline]
    pub fn new(data: &'a mut DataReader<'b>) -> Result<Self, DataError> {
        let len = data.read_fixed::<u32>()?;
        Ok(MapAccess {
            data,
            len,
            current: 0,
        })
    }
}

impl<'de, 'a, 'b: 'de> serde::de::MapAccess<'de> for MapAccess<'a, 'b> {
    type Error = DataError;

    #[inline]
    fn next_key_seed<K>(
        &mut self,
        seed: K,
    ) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.current < self.len {
            let r = seed.deserialize(&mut *self.data)?;
            self.current += 1;
            Ok(Some(r))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn next_value_seed<V>(
        &mut self,
        seed: V,
    ) -> Result<<V as DeserializeSeed<'de>>::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.data)
    }
}

struct VariantAccess<'a, 'b> {
    data: &'a mut DataReader<'b>,
}

impl<'a, 'b> VariantAccess<'a, 'b> {
    #[inline]
    pub fn new(data: &'a mut DataReader<'b>) -> Result<Self, DataError> {
        Ok(VariantAccess { data })
    }
}

impl<'de, 'a, 'b: 'de> serde::de::EnumAccess<'de> for VariantAccess<'a, 'b> {
    type Error = DataError;
    type Variant = Self;
    #[inline]
    fn variant_seed<V>(
        self,
        seed: V,
    ) -> Result<(<V as DeserializeSeed<'de>>::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut *self.data)?, self))
    }
}

impl<'de, 'a, 'b: 'de> serde::de::VariantAccess<'de> for VariantAccess<'a, 'b> {
    type Error = DataError;

    #[inline]
    fn unit_variant(self) -> Result<(), Self::Error> {
        serde::de::Deserialize::deserialize(self.data)
    }

    #[inline]
    fn newtype_variant_seed<T>(
        self,
        seed: T,
    ) -> Result<<T as DeserializeSeed<'de>>::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.data)
    }

    #[inline]
    fn tuple_variant<V>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_seq(self.data, visitor)
    }

    #[inline]
    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_struct(self.data, "", fields, visitor)
    }
}
