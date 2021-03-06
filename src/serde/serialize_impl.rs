use crate::serde::error::DataError;
use crate::Data;
use serde::ser;
use serde::Serialize;
use std::io::Write;

pub struct DataSerializeSeq<'a> {
    write_mode: u8,
    write_data: Option<Data>,
    data: &'a mut Data,
}

impl<'a> DataSerializeSeq<'a> {
    #[inline]
    pub fn new(data: &'a mut Data, len: Option<usize>) -> DataSerializeSeq<'a> {
        if let Some(len) = len {
            data.write_fixed(len as u32);
            DataSerializeSeq {
                write_mode: 0u8,
                write_data: None,
                data,
            }
        } else {
            DataSerializeSeq {
                write_mode: 1u8,
                write_data: Some(Data::new()),
                data,
            }
        }
    }
}

impl<'a> ser::SerializeSeq for DataSerializeSeq<'a> {
    type Ok = ();
    type Error = DataError;
    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.write_mode == 0 {
            value.serialize(&mut *self.data)
        } else {
            if let Some(ref mut table) = self.write_data {
                value.serialize(table)?;
            }
            Ok(())
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.write_mode == 0 {
            Ok(())
        } else {
            if let Some(ref table) = self.write_data {
                self.data.write_fixed(table.len() as u32);
                self.data.write_all(table)?;
            }
            Ok(())
        }
    }
}

pub struct DataSerializeTuple<'a> {
    data: &'a mut Data,
}

impl<'a> DataSerializeTuple<'a> {
    #[inline]
    pub fn new(data: &'a mut Data, len: usize) -> DataSerializeTuple<'_> {
        data.write_fixed(len as u32);
        DataSerializeTuple { data }
    }
}

impl<'a> ser::SerializeTuple for DataSerializeTuple<'a> {
    type Ok = ();
    type Error = DataError;
    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.data)
    }
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct DataSerializeTupleStruct<'a> {
    data: &'a mut Data,
}

impl<'a> DataSerializeTupleStruct<'a> {
    #[inline]
    pub fn new(data: &'a mut Data, len: usize) -> DataSerializeTupleStruct<'a> {
        data.write_fixed(len as u32);
        DataSerializeTupleStruct { data }
    }
}

impl<'a> ser::SerializeTupleStruct for DataSerializeTupleStruct<'a> {
    type Ok = ();
    type Error = DataError;
    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.data)
    }
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for DataSerializeTupleStruct<'a> {
    type Ok = ();
    type Error = DataError;
    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.data.write_fixed(key);
        value.serialize(&mut *self.data)
    }
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct DataSerializeTupleVariant<'a> {
    data: &'a mut Data,
}

impl<'a> DataSerializeTupleVariant<'a> {
    #[inline]
    pub fn new(
        data: &'a mut Data,
        variant: &'static str,
        len: usize,
    ) -> DataSerializeTupleVariant<'a> {
        data.write_fixed(variant);
        data.write_fixed(len as u32);
        DataSerializeTupleVariant { data }
    }
}

impl<'a> ser::SerializeTupleVariant for DataSerializeTupleVariant<'a> {
    type Ok = ();
    type Error = DataError;
    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.data)
    }
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for DataSerializeTupleVariant<'a> {
    type Ok = ();
    type Error = DataError;
    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.data.write_fixed(key);
        value.serialize(&mut *self.data)
    }
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct DataSerializeMap<'a> {
    write_mode: u8,
    write_key: Option<Vec<Data>>,
    write_value: Option<Vec<Data>>,
    data: &'a mut Data,
}

impl<'a> DataSerializeMap<'a> {
    #[inline]
    pub fn new(data: &'a mut Data, len: Option<usize>) -> DataSerializeMap<'a> {
        if let Some(len) = len {
            data.write_fixed(len as u32);
            DataSerializeMap {
                write_mode: 0u8,
                write_key: None,
                write_value: None,
                data,
            }
        } else {
            DataSerializeMap {
                write_mode: 1u8,
                write_key: Some(Vec::new()),
                write_value: Some(Vec::new()),
                data,
            }
        }
    }
}

impl<'a> ser::SerializeMap for DataSerializeMap<'a> {
    type Ok = ();
    type Error = DataError;
    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.write_mode == 0 {
            key.serialize(&mut *self.data)
        } else {
            if let Some(ref mut table) = self.write_key {
                let mut data = Data::new();
                key.serialize(&mut data)?;
                table.push(data);
            }
            Ok(())
        }
    }
    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.write_mode == 0 {
            value.serialize(&mut *self.data)
        } else {
            if let Some(ref mut table) = self.write_value {
                let mut data = Data::new();
                value.serialize(&mut data)?;
                table.push(data);
            }
            Ok(())
        }
    }
    #[inline]
    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        if self.write_mode == 0 {
            Ok(())
        } else {
            if let Some(ref mut keys) = self.write_key {
                if let Some(ref mut values) = self.write_value {
                    assert_eq!(keys.len(), values.len());
                    self.data.write_fixed(keys.len() as u32);
                    for i in 0..keys.len() {
                        let key = keys.get(i).unwrap();
                        let value = values.get(i).unwrap();
                        self.data.write_all(key)?;
                        self.data.write_all(value)?;
                    }
                }
            }
            Ok(())
        }
    }
}
