use anyhow::{bail, ensure, Result};
use std::convert::TryInto;
use std::mem::size_of;
use std::ops::Deref;

pub trait ReadNumberFixed {
    fn read(dr: &mut DataReader) -> Result<Self>
    where
        Self: Sized;
}

pub trait ReadNumberVar {
    fn read(dr: &mut DataReader) -> Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_read_number_fixed {
    ($type:tt) => {
        impl ReadNumberFixed for $type {
            #[cfg(not(feature = "big_endian"))]
            #[inline]
            fn read(dr: &mut DataReader) -> Result<Self>
            where
                Self: Sized,
            {
                let size = size_of::<$type>();
                ensure!(
                    size <= dr.len(),
                    "read fixed error len too min:dr:{} < {}",
                    dr.len(),
                    size
                );
                let v = $type::from_le_bytes(dr[..size].try_into()?);
                dr.advance(size)?;
                Ok(v)
            }

            #[cfg(feature = "big_endian")]
            #[inline]
            fn read(dr: &mut DataReader) -> Result<Self>
            where
                Self: Sized,
            {
                let size = size_of::<$type>();
                ensure!(
                    size <= dr.len(),
                    "read fixed error len too min:dr:{} < {}",
                    dr.len(),
                    size
                );
                let v = $type::from_be_bytes(dr[..size].try_into()?);
                dr.advance(size)?;
                Ok(v)
            }
        }
    };
}

impl ReadNumberFixed for bool {
    #[inline]
    fn read(dr: &mut DataReader) -> Result<Self>
    where
        Self: Sized,
    {
        let v = dr.read_fixed::<u8>()?;
        if v == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

impl_read_number_fixed!(u8);
impl_read_number_fixed!(i8);
impl_read_number_fixed!(u16);
impl_read_number_fixed!(i16);
impl_read_number_fixed!(u32);
impl_read_number_fixed!(i32);
impl_read_number_fixed!(u64);
impl_read_number_fixed!(i64);
impl_read_number_fixed!(f32);
impl_read_number_fixed!(f64);

impl ReadNumberVar for u16 {
    #[inline]
    fn read(dr: &mut DataReader) -> Result<Self>
    where
        Self: Sized,
    {
        let mut v = 0u16;
        let mut offset = 0;
        let mut shift = 0u8;
        let mut b;
        while shift < 16 {
            ensure!(
                offset != dr.len(),
                "read var number,offset:{} > bytes length:{}",
                offset,
                dr.len()
            );
            b = dr[offset];
            offset += 1;
            v |= ((b & 0x7F) as u16) << shift;
            if b & 0x80 == 0 {
                dr.buff = &dr.buff[offset..];
                return Ok(v);
            }
            shift += 7;
        }
        bail!("not read var number too end")
    }
}

impl ReadNumberVar for i16 {
    #[inline]
    fn read(dr: &mut DataReader) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(zig_zag_decode_i16(ReadNumberVar::read(dr)?))
    }
}

impl ReadNumberVar for u32 {
    #[inline]
    fn read(dr: &mut DataReader) -> Result<Self>
    where
        Self: Sized,
    {
        let mut v = 0u32;
        let mut offset = 0;
        let mut shift = 0u8;
        let mut b;
        while shift < 32 {
            ensure!(
                offset != dr.len(),
                "read var number,offset:{} > bytes length:{}",
                offset,
                dr.len()
            );
            b = dr[offset];
            offset += 1;
            v |= ((b & 0x7F) as u32) << shift;
            if b & 0x80 == 0 {
                dr.buff = &dr.buff[offset..];
                return Ok(v);
            }
            shift += 7;
        }
        bail!("not read var number too end")
    }
}

impl ReadNumberVar for i32 {
    #[inline]
    fn read(dr: &mut DataReader) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(zig_zag_decode_i32(ReadNumberVar::read(dr)?))
    }
}

impl ReadNumberVar for u64 {
    #[inline]
    fn read(dr: &mut DataReader) -> Result<Self>
    where
        Self: Sized,
    {
        let mut v = 0u64;
        let mut offset = 0;
        let mut shift = 0u8;
        let mut b;
        while shift < 64 {
            ensure!(
                offset != dr.len(),
                "read var number,offset:{} > bytes length:{}",
                offset,
                dr.len()
            );
            b = dr[offset];
            offset += 1;
            v |= ((b & 0x7F) as u64) << shift;
            if b & 0x80 == 0 {
                dr.buff = &dr.buff[offset..];
                return Ok(v);
            }
            shift += 7;
        }
        bail!("not read var number too end")
    }
}

impl ReadNumberVar for i64 {
    #[inline]
    fn read(dr: &mut DataReader) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(zig_zag_decode_i64(ReadNumberVar::read(dr)?))
    }
}

#[inline(always)]
fn zig_zag_decode_i16(v: u16) -> i16 {
    ((v >> 1) as i16) ^ (-((v & 1) as i16))
}
#[inline(always)]
fn zig_zag_decode_i32(v: u32) -> i32 {
    ((v >> 1) as i32) ^ (-((v & 1) as i32))
}
#[inline(always)]
fn zig_zag_decode_i64(v: u64) -> i64 {
    ((v >> 1) as i64) ^ (-((v & 1) as i64))
}

#[derive(Debug)]
pub struct DataReader<'a> {
    pub(crate) buff: &'a [u8],
    pub(crate) original_len: usize,
    pub(crate) mode: u8,
}

impl<'a> From<&'a [u8]> for DataReader<'a> {
    #[inline]
    fn from(buff: &'a [u8]) -> Self {
        DataReader {
            buff,
            original_len: buff.len(),
            mode: 0,
        }
    }
}

impl<'a> Deref for DataReader<'a> {
    type Target = &'a [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.buff
    }
}

impl<'a> AsRef<[u8]> for DataReader<'a>{
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.buff
    }
}


impl<'a> DataReader<'a> {

    pub fn from<T:AsRef<[u8]> + ?Sized>(v:&'a T)->Self{
        let buff=v.as_ref();
        DataReader {
            original_len: buff.len(),
            buff,
            mode: 0,
        }
    }

    #[inline]
    pub fn advance(&mut self, cnt: usize) -> Result<()> {
        ensure!(
            self.len() >= cnt,
            "advance error,cnt:{} > len:{}",
            cnt,
            self.len()
        );
        self.buff = &self.buff[cnt..];
        Ok(())
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.original_len.wrapping_sub(self.buff.len())
    }

    #[inline]
    pub fn reload(&mut self,buff: &'a [u8],original_len:usize){
        self.buff=buff;
        self.original_len=original_len;
    }

    #[inline]
    pub fn read_buff(&mut self, buff: &mut [u8]) -> Result<()> {
        let size = buff.len();
        ensure!(
            self.len() >= size,
            "read buff,buff too max,current:{} input:{}",
            self.len(),
            size
        );
        let (copy, current) = self.buff.split_at(size);
        buff.copy_from_slice(copy);
        self.buff = current;
        Ok(())
    }

    #[inline]
    pub fn read_var_str(&mut self) -> Result<&'a str> {
        let len = self.read_var_integer::<u64>()? as usize;
        ensure!(
            len <= self.len(),
            "read string size too big,{}>{}",
            len,
            self.len()
        );
        let (res, have) = self.buff.split_at(len);
        self.buff = have;

        cfg_if::cfg_if!{
            if #[cfg(feature ="check_utf8")]{
                 Ok(std::str::from_utf8(res)?)
            }else{
               unsafe {
                    Ok(std::str::from_utf8_unchecked(res))
               }
            }
        }
    }

    #[inline]
    pub fn read_fixed_str(&mut self) -> Result<&'a str> {
        let len = self.read_fixed::<u32>()? as usize;
        ensure!(
            len <= self.len(),
            "read string size too big,{}>{}",
            len,
            self.len()
        );
        let (res, have) = self.buff.split_at(len);
        self.buff = have;

        cfg_if::cfg_if!{
            if #[cfg(feature ="check_utf8")]{
                 Ok(std::str::from_utf8(res)?)
            }else{
               unsafe {
                    Ok(std::str::from_utf8_unchecked(res))
               }
            }
        }
    }

    #[inline]
    pub fn read_var_buf(&mut self) -> Result<&'a [u8]> {
        let len = self.read_var_integer::<u64>()? as usize;
        ensure!(
            len <= self.len(),
            "read string size too big,{}>{}",
            len,
            self.len()
        );
        let (res, have) = self.buff.split_at(len);
        self.buff = have;
        Ok(res)
    }

    #[inline]
    pub fn read_fixed_buf(&mut self) -> Result<&'a [u8]> {
        let len = self.read_fixed::<u32>()? as usize;
        ensure!(
            len <= self.len(),
            "read string size too big,{}>{}",
            len,
            self.len()
        );
        let (res, have) = self.buff.split_at(len);
        self.buff = have;
        Ok(res)
    }

    #[inline]
    pub fn read_fixed<T: ReadNumberFixed>(&mut self) -> Result<T> {
        T::read(self)
    }

    #[inline]
    pub fn read_var_integer<T: ReadNumberVar>(&mut self) -> Result<T> {
        T::read(self)
    }
}
