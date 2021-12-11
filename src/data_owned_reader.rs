use crate::{DataReader, ReadNumberFixed, ReadNumberVar};
use anyhow::{ensure, Result};
use std::ops::Deref;

#[derive(Debug)]
pub struct DataOwnedReader{
    pub(crate) inner:Vec<u8>,
    pub(crate) offset:usize
}

impl DataOwnedReader{
    #[inline]
    pub fn new(inner:Vec<u8>)->DataOwnedReader{
        DataOwnedReader{
            inner,
            offset:0
        }
    }

    #[inline]
    pub fn into_inner(self)->Vec<u8>{
        self.inner
    }
    #[inline]
    pub fn get_reader(&self)->DataReader{
        DataReader::from(&self.inner[self.offset..])
    }

    #[inline]
    pub fn get_all_reader(&self)->DataReader{
        DataReader::from(&self.inner[..])
    }
    #[inline] #[inline]
    pub fn get_offset(&self)->usize{
        self.offset
    }

    #[inline]
    pub fn set_offset(&mut self,offset:usize)->Result<()>{
        ensure!(offset<=self.inner.len(),"offset big too");
        self.offset=offset;
        Ok(())
    }

    #[inline]
    pub fn add_offset(&mut self,offset:usize)->Result<usize>{
        ensure!( self.offset+offset<=self.inner.len(),"offset big too");
        self.offset+=offset;
        Ok(self.offset)
    }
    #[inline]
    pub fn sub_offset(&mut self,offset:usize)->Result<usize>{
        ensure!(self.offset-offset>0,"offset min too");
        self.offset-=offset;
        Ok(self.offset)
    }


    #[inline]
    pub fn read_fixed<T: ReadNumberFixed>(&mut self) -> Result<T> {
        let mut dr=self.get_reader();
        let v=dr.read_fixed()?;
        self.offset+=dr.offset();
        Ok(v)
    }

    #[inline]
    pub fn read_var_integer<T: ReadNumberVar>(&mut self) -> Result<T> {
        let mut dr=self.get_reader();
        let v=dr.read_var_integer()?;
        self.offset+=dr.offset();
        Ok(v)
    }

    #[inline]
    pub fn read_fixed_buf(&mut self) -> Result<&[u8]> {
        let mut dr=DataReader::from(&self.inner[self.offset..]);
        let x= dr.read_fixed_buf()?;
        self.offset+=dr.offset();
        Ok(x)
    }

    #[inline]
    pub fn read_var_buf(&mut self) -> Result<&[u8]> {
        let mut dr=DataReader::from(&self.inner[self.offset..]);
        let x= dr.read_var_buf()?;
        self.offset+=dr.offset();
        Ok(x)
    }

    #[inline]
    pub fn read_fixed_str(&mut self) -> Result<&str> {
        let mut dr=DataReader::from(&self.inner[self.offset..]);
        let x= dr.read_fixed_str()?;
        self.offset+=dr.offset();
        Ok(x)
    }

    #[inline]
    pub fn read_var_str(&mut self) -> Result<&str> {
        let mut dr=DataReader::from(&self.inner[self.offset..]);
        let x= dr.read_var_str()?;
        self.offset+=dr.offset();
        Ok(x)
    }

    #[inline]
    pub fn read_buff(&mut self, buff: &mut [u8]) -> Result<()> {
        let mut dr=DataReader::from(&self.inner[self.offset..]);
        dr.read_buff(buff)?;
        self.offset+=dr.offset();
        Ok(())
    }

}

impl Deref for DataOwnedReader{
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<[u8]> for DataOwnedReader{
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}