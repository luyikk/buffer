use crate::Data;
use serde::{Serialize, Deserialize};
use anyhow::*;

impl Data{
    #[cfg(all(feature = "rmp",feature = "json"))]
    pub fn pack_to<'a,T:Deserialize<'a>>(&'a mut self) ->Result<T>{
        Ok(rmp_serde::decode::from_read_ref(&self[..])?)
    }

    #[cfg(all(feature = "rmp",feature = "json"))]
    pub fn pack_from<T: Serialize>(value:T) ->Result<Data> {
        Ok(rmp_serde::encode::to_vec(&value)?.into())
    }

    #[cfg(all(feature = "rmp",feature = "json"))]
    pub fn pack_deserialize<'a,T:Deserialize<'a>>(&'a mut self) ->Result<T> {
        let len = self.get_le::<u32>()? as usize;
        let start = self.offset;
        if !self.set_position(start + len) {
            bail!("index overflow {}", line!())
        }
        Ok(rmp_serde::decode::from_read_ref(&self[start..self.offset])?)
    }
    #[cfg(all(feature = "rmp",feature = "json"))]
    pub fn pack_serialize<T:Serialize>(&mut self, value:T) ->Result<()> {
        let buff = rmp_serde::encode::to_vec(&value)?;
        self.serde_serialize(buff)?;
        Ok(())
    }

    #[cfg(all(feature = "json",not(feature = "rmp")))]
    pub fn pack_to<'a,T:Deserialize<'a>>(&'a mut self) ->Result<T>{
        Ok(serde_json::from_slice(&self[..])?)
    }

    #[cfg(all(feature = "json",not(feature = "rmp")))]
    pub fn pack_from<T: Serialize>(value:T) ->Result<Data> {
        Ok(serde_json::to_vec(&value)?.into())
    }

    #[cfg(all(feature = "json",not(feature = "rmp")))]
    pub fn pack_deserialize<'a,T:Deserialize<'a>>(&'a mut self) ->Result<T> {
        let len = self.get_le::<u32>()? as usize;
        let start = self.offset;
        if !self.set_position(start + len) {
            bail!("index overflow {}", line!())
        }
        Ok(serde_json::from_slice(&self[start..self.offset])?)
    }

    #[cfg(all(feature = "json",not(feature = "rmp")))]
    pub fn pack_serialize<T:Serialize>(&mut self, value:T) ->Result<()> {
        let buff = serde_json::to_vec(&value)?;
        self.serde_serialize(buff)?;
        Ok(())
    }

}
