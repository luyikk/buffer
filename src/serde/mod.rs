use crate::{Data, DataReader};
use serde::{Deserialize, Serialize};
use crate::serde::error::DataError;
use crate::data_owned_reader::DataOwnedReader;

pub mod de;
pub mod error;
pub mod ser;
pub mod serialize_impl;


impl Data {
    #[inline]
    pub fn serde_serialize<T: Serialize>(&mut self, value: T) ->Result<(),DataError> {
        value.serialize(self)?;
        Ok(())
    }
}

impl<'a, 'de, 'b> DataReader<'b>
where
    'b: 'de,
{
    #[inline]
    pub fn serde_deserialize<T: Deserialize<'de>>(&'a mut self) ->Result<T,DataError> {
         T::deserialize(self)
    }
}

impl DataOwnedReader{
    #[inline]
    pub fn serde_deserialize<'de,T:Deserialize<'de>>(&'de mut self)->Result<T,DataError>{
        let mut dr=DataReader::from(&self.inner[self.offset..]);
        let v= dr.serde_deserialize()?;
        self.offset+=dr.offset();
        Ok(v)
    }
}