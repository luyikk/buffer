use crate::data_owned_reader::DataOwnedReader;
use crate::{Data, DataReader};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 这是为了兼容而写的,请不要随便使用
/// This is written for compatibility. Please don't use it casually

impl Data {
    #[cfg(all(feature = "rmp", not(feature = "json"), not(feature = "data")))]
    #[inline]
    pub fn pack_from<T: Serialize>(value: T) -> Result<Data> {
        Ok(rmp_serde::encode::to_vec(&value)?.into())
    }

    #[cfg(all(feature = "rmp", not(feature = "json"), not(feature = "data")))]
    #[inline]
    pub fn pack_serialize<T: Serialize>(&mut self, value: T) -> Result<()> {
        self.mode = 1;
        let bak = self.len();
        if self.serde_serialize(&value).is_err() {
            self.truncate(bak);
            self.mode = 0;
            let buff = rmp_serde::encode::to_vec(&value)?;
            self.write_fixed(buff);
        }
        Ok(())
    }

    #[cfg(all(feature = "json", feature = "rmp"))]
    #[inline]
    pub fn pack_from<T: Serialize>(value: T) -> Result<Data> {
        Ok(serde_json::to_vec(&value)?.into())
    }

    #[cfg(all(feature = "json", feature = "rmp"))]
    #[inline]
    pub fn pack_serialize<T: Serialize>(&mut self, value: T) -> Result<()> {
        self.mode = 1;
        let bak = self.len();
        if self.serde_serialize(&value).is_err() {
            self.truncate(bak);
            self.mode = 0;
            let buff = serde_json::to_vec(&value)?;
            self.write_fixed(buff);
        }
        self.mode = 0;
        Ok(())
    }

    #[cfg(all(feature = "rmp", feature = "data"))]
    #[inline]
    pub fn pack_from<T: Serialize>(value: T) -> Result<Data> {
        let mut data = Data::new();
        data.pack_serialize(value)?;
        Ok(data)
    }

    #[cfg(all(feature = "rmp", feature = "data"))]
    #[inline]
    pub fn pack_serialize<T: Serialize>(&mut self, value: T) -> Result<()> {
        self.mode = 0;
        self.serde_serialize(&value)?;
        Ok(())
    }
}

impl<'de, 'a: 'de> DataReader<'a> {
    #[cfg(all(feature = "rmp", not(feature = "json"), not(feature = "data")))]
    #[inline]
    pub fn pack_to<'b, T: Deserialize<'de>>(&'b mut self) -> Result<T> {
        Ok(rmp_serde::decode::from_slice(self.buff)?)
    }

    #[cfg(all(feature = "rmp", not(feature = "json"), not(feature = "data")))]
    #[inline]
    pub fn pack_deserialize<'b, T: Deserialize<'de>>(&'b mut self) -> Result<T> {
        let mut check_buff = DataReader::from(self.buff);
        check_buff.mode = 1;
        match check_buff.serde_deserialize() {
            Ok(value) => {
                self.reload(check_buff.buff, check_buff.original_len);
                Ok(value)
            }
            Err(_) => {
                let buff = self.read_fixed_buf()?;
                Ok(rmp_serde::decode::from_slice(buff)?)
            }
        }
    }

    #[cfg(all(feature = "json", feature = "rmp"))]
    #[inline]
    pub fn pack_to<'b, T: Deserialize<'de>>(&'b mut self) -> Result<T> {
        Ok(serde_json::from_slice(self.buff)?)
    }

    #[cfg(all(feature = "json", feature = "rmp"))]
    #[inline]
    pub fn pack_deserialize<'b, T: Deserialize<'de>>(&'b mut self) -> Result<T> {
        let mut check_buff = DataReader::from(self.buff);
        check_buff.mode = 1;
        match check_buff.serde_deserialize() {
            Ok(value) => {
                self.reload(check_buff.buff, check_buff.original_len);
                Ok(value)
            }
            Err(_) => {
                let buff = self.read_fixed_buf()?;
                Ok(serde_json::from_slice(buff)?)
            }
        }
    }

    #[cfg(all(feature = "data", feature = "rmp"))]
    #[inline]
    pub fn pack_to<'b, T: Deserialize<'de>>(&'b mut self) -> Result<T> {
        let mut rdata = DataReader::from(self.buff);
        Ok(rdata.serde_deserialize()?)
    }

    #[cfg(all(feature = "data", feature = "rmp"))]
    #[inline]
    pub fn pack_deserialize<'b, T: Deserialize<'de>>(&'b mut self) -> Result<T> {
        let mut check_buff = DataReader::from(self.buff);
        check_buff.mode = 0;
        let value = check_buff.serde_deserialize()?;
        self.reload(check_buff.buff, check_buff.original_len);
        Ok(value)
    }
}

impl DataOwnedReader {
    #[cfg(all(feature = "rmp", not(feature = "json"), not(feature = "data")))]
    #[inline]
    pub fn pack_to<'a, T: Deserialize<'a>>(&'a mut self) -> Result<T> {
        Ok(rmp_serde::decode::from_slice(self)?)
    }

    #[cfg(all(feature = "rmp", not(feature = "json"), not(feature = "data")))]
    #[inline]
    pub fn pack_deserialize<'a, T: Deserialize<'a>>(&'a mut self) -> Result<T> {
        let mut dr = DataReader::from(&self.inner[self.offset..]);
        dr.mode = 1;
        match dr.serde_deserialize() {
            Ok(value) => {
                self.offset += dr.offset();
                Ok(value)
            }
            Err(_) => {
                dr = DataReader::from(&self.inner[self.offset..]);
                let buff = dr.read_fixed_buf()?;
                self.offset += dr.offset();
                Ok(rmp_serde::decode::from_slice(buff)?)
            }
        }
    }

    #[cfg(all(feature = "rmp", feature = "json"))]
    #[inline]
    pub fn pack_to<'a, T: Deserialize<'a>>(&'a mut self) -> Result<T> {
        Ok(serde_json::from_slice(self)?)
    }

    #[cfg(all(feature = "rmp", feature = "json"))]
    #[inline]
    pub fn pack_deserialize<'a, T: Deserialize<'a>>(&'a mut self) -> Result<T> {
        let mut dr = DataReader::from(&self.inner[self.offset..]);
        dr.mode = 1;
        match dr.serde_deserialize() {
            Ok(value) => {
                self.offset += dr.offset();
                Ok(value)
            }
            Err(_) => {
                dr = DataReader::from(&self.inner[self.offset..]);
                let buff = dr.read_fixed_buf()?;
                self.offset += dr.offset();
                Ok(serde_json::from_slice(buff)?)
            }
        }
    }

    #[cfg(all(feature = "rmp", feature = "data"))]
    #[inline]
    pub fn pack_to<'a, T: Deserialize<'a>>(&'a mut self) -> Result<T> {
        let mut rdata = DataReader::from(self);
        Ok(rdata.serde_deserialize()?)
    }

    #[cfg(all(feature = "rmp", feature = "data"))]
    #[inline]
    pub fn pack_deserialize<'a, T: Deserialize<'a>>(&'a mut self) -> Result<T> {
        let mut dr = DataReader::from(&self.inner[self.offset..]);
        dr.mode = 0;
        let value = dr.serde_deserialize()?;
        self.offset += dr.offset();
        Ok(value)
    }
}
