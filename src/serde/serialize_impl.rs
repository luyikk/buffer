use crate::Data;
use serde::{ser, Serialize};

use crate::serde::error::DataError;

pub struct DataSerializeSeq<'a> {
    write_mode:u8,
    write_data:Option<Vec<Data>>,
    data:&'a mut Data,
}

impl<'a> DataSerializeSeq<'a>{
    pub fn new(data:&'a mut Data, len:Option<usize>)->DataSerializeSeq<'a>{

        if let Some(len)=len {
            data.write_to_le(&(len as u32));
            DataSerializeSeq {
                write_mode:0u8,
                write_data:None,
                data
            }
        }
        else{
            DataSerializeSeq {
                write_mode:1u8,
                write_data:Some(Vec::new()),
                data
            }
        }
    }
}

impl<'a> ser::SerializeSeq for DataSerializeSeq<'a>{
    type Ok = ();
    type Error = DataError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        if self.write_mode==0 {
            value.serialize(&mut *self.data)
        }
        else{
            if let Some(ref mut table)= self.write_data{
                let mut data=Data::new();
                value.serialize(&mut data)?;
                table.push(data);
            }
            Ok(())
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        if self.write_mode==0 {
            Ok(())
        }else{
            if let Some(ref mut table)= self.write_data {
                self.data.write_to_le(&(table.len() as u32));
                for v in table {
                    self.data.write(v);
                }
            }
            Ok(())
        }
    }
}




pub struct DataSerializeTuple<'a>{
    data:&'a mut Data
}

impl<'a> DataSerializeTuple<'a>{
    pub fn new(data:&'a mut Data, len:usize)->DataSerializeTuple<'_>{
        data.write_to_le(&(len as u32));
        DataSerializeTuple{
            data
        }
    }
}

impl<'a> ser::SerializeTuple for DataSerializeTuple<'a>{
    type Ok = ();
    type Error =DataError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize( &mut *self.data)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
       Ok(())
    }
}

pub struct DataSerializeTupleStruct<'a>{
    data:&'a mut Data
}

impl<'a> DataSerializeTupleStruct<'a>{
    pub fn new(data:&'a mut Data, len:usize)->DataSerializeTupleStruct<'a>{
        data.write_to_le(&(len as u32));
        DataSerializeTupleStruct{
            data
        }
    }
}

impl<'a> ser::SerializeTupleStruct for DataSerializeTupleStruct<'a>{
    type Ok = ();
    type Error =DataError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize( &mut *self.data)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for DataSerializeTupleStruct<'a>{
    type Ok = ();
    type Error =DataError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.data.write_to_le(&key);
        value.serialize( &mut *self.data)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}


pub struct  DataSerializeTupleVariant<'a> {
    data:&'a mut Data
}

impl<'a> DataSerializeTupleVariant<'a>{
    pub fn new(data:&'a mut Data, variant:&'static str, len:usize)->DataSerializeTupleVariant<'a>{
        data.write_to_le(&variant);
        data.write_to_le(&(len as u32));
        DataSerializeTupleVariant{
            data
        }
    }
}

impl<'a> ser::SerializeTupleVariant for DataSerializeTupleVariant<'a>{
    type Ok = ();
    type Error = DataError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize(&mut *self.data)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl <'a> ser::SerializeStructVariant for DataSerializeTupleVariant<'a>{
    type Ok = ();
    type Error = DataError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.data.write_to_le(&key);
        value.serialize( &mut *self.data)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct DataSerializeMap<'a>{
    write_mode:u8,
    write_key:Option<Vec<Data>>,
    write_value:Option<Vec<Data>>,
    data:&'a mut Data
}

impl <'a> DataSerializeMap<'a>{
    pub fn new(data:&'a mut Data, len:Option<usize>)->DataSerializeMap<'a>{
        if let Some(len)=len {
            data.write_to_le(&(len as u32));
            DataSerializeMap {
                write_mode:0u8,
                write_key:None,
                write_value:None,
                data
            }
        }
        else{
            DataSerializeMap {
                write_mode:1u8,
                write_key:Some(Vec::new()),
                write_value:Some(Vec::new()),
                data
            }
        }
    }
}

impl <'a> ser::SerializeMap for DataSerializeMap<'a>{
    type Ok = ();
    type Error = DataError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where
        T: Serialize {
        if self.write_mode==0 {
            key.serialize(&mut *self.data)
        }
        else{
            if let Some(ref mut table)= self.write_key{
                let mut data=Data::new();
                key.serialize(&mut data)?;
                table.push(data);
            }
            Ok(())
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        if self.write_mode==0 {
            value.serialize(&mut *self.data)
        }
        else{
            if let Some(ref mut table)= self.write_value{
                let mut data=Data::new();
                value.serialize(&mut data)?;
                table.push(data);
            }
            Ok(())
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        if self.write_mode==0 {
            Ok(())
        }else{
            if let Some(ref mut keys)= self.write_key{
                if let Some(ref mut values)=self.write_value {
                    assert_eq!(keys.len(),values.len());
                    self.data.write_to_le(&(keys.len() as u32));
                    for i in 0..keys.len() {
                        let key=keys.get(i).unwrap();
                        let value=values.get(i).unwrap();
                        self.data.write(key);
                        self.data.write(value);
                    }

                }
            }
            Ok(())
        }
    }
}

