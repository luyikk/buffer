#![feature(optin_builtin_traits,negative_impls)]

mod wr;
mod into;
mod readfrom;
mod todata;

use bytes::buf::UninitSlice;
use bytes::{Buf, BufMut};
use std::io;
use std::io::{ErrorKind};
use std::ops::{Deref, DerefMut};
pub use wr::*;
pub use into::*;
pub use readfrom::*;
pub use todata::*;

#[derive(Debug,Clone)]
pub struct Data {
    buf: Vec<u8>,
    offset: usize
}

unsafe impl BufMut for Data {
    #[inline]
    fn remaining_mut(&self) -> usize {
        self.buf.remaining_mut()
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.buf.advance_mut(cnt);
    }

    #[inline]
    fn bytes_mut(&mut self) -> &mut UninitSlice {
        self.buf.bytes_mut()
    }
}

impl Buf for Data {
    #[inline]
    fn remaining(&self) -> usize {
        if self.buf.len() > self.offset {
            self.buf.len() - self.offset
        } else {
            0
        }
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        let len = self.remaining();

        if len > 0 {
            &self.buf[self.offset..]
        } else {
            &[]
        }
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        self.offset += cnt;
    }
}

impl Default for Data {
    #[inline]
    fn default() -> Self {
        Data {
            buf: Vec::new(),
            offset: 0
        }
    }
}

impl Deref for Data{
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.buf.deref()
    }
}

impl DerefMut for Data{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buf.deref_mut()
    }
}

impl From<Vec<u8>> for Data{
    #[inline]
    fn from(data: Vec<u8>) -> Self {
        Data {
            buf: data,
            offset: 0
        }
    }
}

impl Data {
    #[inline]
    pub fn new() -> Data {
        Data::default()
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Data {
        Data {
            buf: Vec::with_capacity(cap),
            offset: 0
        }
    }

    #[inline]
    pub fn with_len(len:usize,default:u8)->Data{
        Data {
            buf: vec![default;len],
            offset: 0
        }
    }

    /// 获取OFFSET
    #[inline(always)]
    pub fn get_position(&self) -> usize {
        self.offset
    }

    /// 设置OFFSET
    #[inline]
    pub fn set_position(&mut self, offset: usize) -> bool {
        if offset > self.buf.len() {
            return false;
        }
        self.offset = offset;
        return true;
    }

    /// 长度
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    #[inline(always)]
    pub fn have_len(&self)->usize{
        self.len()-self.offset
    }
    /// 重置
    #[inline]
    pub fn reset(&mut self) {
        self.buf.resize(0, 0);
        self.offset = 0;
    }

    /// 重置长度
    #[inline]
    pub fn resize(&mut self,new_len:usize,value:u8) {
        self.buf.resize(new_len, value);
    }

    /// 清理
    #[inline]
    pub fn clear(&mut self) {
        self.buf.clear();
        self.offset = 0;
    }

    /// 写入buff
    #[inline]
    pub fn write(&mut self, buff: &[u8]) {
        //self.buf.extend_from_slice(buff)
        self.write_ptr(buff.as_ptr(),buff.len());
    }

    #[inline]
    pub fn write_ptr(&mut self, buff: *const u8, len: usize) {
        if self.buf.len() + len > self.buf.capacity() {
            self.buf.reserve(len);
        }
        unsafe {
            let end = self.buf.as_mut_ptr().add(self.buf.len());
            end.copy_from(buff, len);
            self.buf.set_len(self.buf.len() + len);
        }
    }

    /// 读取buff
    #[inline]
    pub fn read(&mut self, len: usize) -> io::Result<Vec<u8>> {
        if self.buf.len() >= self.offset + len {
            let data = self.buf[self.offset..self.offset + len].to_vec();
            self.offset += len;
            Ok(data)
        } else {
            Err(io::Error::new(ErrorKind::InvalidData,format!("offset + len too big line:{}",line!())))
        }
    }
    /// 写入变成U16
    #[inline]
    pub fn bit7_write_u16(&mut self, value: &u16) {
        let mut v = *value;
        while v >= 1 << 7 {
            self.buf.push((v & 0x7f | 0x80) as u8);
            v = v >> 7;
        }
        self.buf.push(v as u8);
    }
    /// 写入变成U32
    #[inline]
    pub fn bit7_write_u32(&mut self, value: &u32) {
        let mut v = *value;
        while v >= 1 << 7 {
            self.buf.push((v & 0x7f | 0x80) as u8);
            v = v >> 7;
        }
        self.buf.push(v as u8);
    }
    /// 写入变成U64
    #[inline]
    pub fn bit7_write_u64(&mut self, value: &u64) {
        let mut v = *value;
        while v >= 1 << 7 {
            self.buf.push((v & 0x7f | 0x80) as u8);
            v = v >> 7;
        }
        self.buf.push(v as u8);
    }
    /// 写入变长i64
    #[inline]
    pub fn bit7_write_i16(&mut self, value: &i16) {
        self.bit7_write_u16(&zig_zag_encode_u16(*value))
    }
    /// 写入变长i32
    #[inline]
    pub fn bit7_write_i32(&mut self, value: &i32) {
        self.bit7_write_u32(&zig_zag_encode_u32(*value))
    }
    /// 写入变长i64
    #[inline]
    pub fn bit7_write_i64(&mut self, value: &i64) {
        self.bit7_write_u64(&zig_zag_encode_u64(*value))
    }

    #[inline]
    pub fn bit7_write_f32(&mut self,value:&f32){
        self.write_to_le::<f32>(value)
    }

    #[inline]
    pub fn bit7_write_f64(&mut self,value:&f64){
        self.write_to_le::<f64>(value)
    }

    #[inline]
    pub fn bit7_write_u128(&mut self,value:&u128){
        self.write_to_le::<u128>(value)
    }

    #[inline]
    pub fn bit7_write_i128(&mut self,value:&i128){
        self.write_to_le::<i128>(value)
    }


    /// 读取变长 u16
    #[inline]
    pub fn read_bit7_u16(&mut self) -> io::Result<(usize, u16)> {
        let mut v = 0;
        let mut offset = self.offset;
        let mut shift = 0;
        while shift < 2 * 8 {
            if offset >= self.buf.len() {
                return Err(io::Error::new(ErrorKind::InvalidData,format!("buff len too small line:{}",line!())));
            }

            let b = self.buf[offset];
            offset += 1;
            v |= ((b & 0x7F) as u16) << shift;
            if b & 0x80 == 0 {
                let len = offset - self.offset;
                self.offset = offset;
                return Ok((len, v));
            }
            shift += 7;
        }
        return Err(io::Error::new(ErrorKind::InvalidData,format!("buff is error line:{}",line!())));
    }
    /// 读取变长i16
    #[inline]
    pub fn read_bit7_i16(&mut self) -> io::Result<(usize, i16)> {
        let (offset, v) = self.read_bit7_u16()?;
        let v = zig_zag_decode_i16(v);
        Ok((offset, v))
    }
    /// 读取变长u32
    #[inline]
    pub fn read_bit7_u32(&mut self) -> io::Result<(usize, u32)> {
        let mut v = 0;
        let mut offset = self.offset;
        let mut shift = 0;
        while shift < 4 * 8 {
            if offset >= self.buf.len() {
                return Err(io::Error::new(ErrorKind::InvalidData,format!("buff len too small line:{}",line!())));
            }

            let b = self.buf[offset];
            offset += 1;
            v |= ((b & 0x7F) as u32) << shift;
            if b & 0x80 == 0 {
                let len = offset - self.offset;
                self.offset = offset;
                return Ok((len, v));
            }
            shift += 7;
        }
        return Err(io::Error::new(ErrorKind::InvalidData,format!("buff is error line:{}",line!())));
    }
    /// 读取变长i32
    #[inline]
    pub fn read_bit7_i32(&mut self) -> io::Result<(usize, i32)> {
        let (offset, v) = self.read_bit7_u32()?;
        let v = zig_zag_decode_i32(v);
        Ok((offset, v))
    }
    /// 读取变长u64
    #[inline]
    pub fn read_bit7_u64(&mut self) -> io::Result<(usize, u64)> {
        let mut v = 0;
        let mut offset = self.offset;
        let mut shift = 0;
        while shift < 8 * 8 {
            if offset >= self.buf.len() {
                return Err(io::Error::new(ErrorKind::InvalidData,format!("buff len too small line:{}",line!())));
            }

            let b = self.buf[offset];
            offset += 1;
            v |= ((b & 0x7F) as u64) << shift;
            if b & 0x80 == 0 {
                let len = offset - self.offset;
                self.offset = offset;
                return Ok((len, v));
            }
            shift += 7;
        }
        return Err(io::Error::new(ErrorKind::InvalidData,format!("buff is error:{}",line!())));
    }
    /// 读取变长i64
    #[inline]
    pub fn read_bit7_i64(&mut self) ->  io::Result<(usize, i64)> {
        let (offset, v) = self.read_bit7_u64()?;
        let v = zig_zag_decode_i64(v);
        Ok((offset, v))
    }


    #[inline]
    pub fn read_bit7_i128(&mut self)->io::Result<(usize,i128)>{
        Ok((16,self.get_le::<i128>()?))
    }

    #[inline]
    pub fn read_bit7_u128(&mut self)->io::Result<(usize,u128)>{
        Ok((16,self.get_le::<u128>()?))
    }

    #[inline]
    pub fn read_bit7_f32(&mut self)->io::Result<(usize,f32)>{
        Ok((4,self.get_le::<f32>()?))
    }

    #[inline]
    pub fn read_bit7_f64(&mut self)->io::Result<(usize,f64)>{
        Ok((8,self.get_le::<f64>()?))
    }

    /// 写入二进制变长长度
    #[inline]
    pub fn write_buff_bit7(&mut self, data: &[u8]) {
        self.bit7_write_u64(&(data.len() as u64));
        self.write(data)
    }

    /// 写入二进制定长长度 u32 le
    #[inline]
    pub fn write_buff_fixed_le(&mut self, data: &[u8]) {
        self.write_to_le(&(data.len() as u32));
        self.write(data)
    }

    /// 写入二进制定长长度 u32
    #[inline]
    pub fn write_buff_fixed(&mut self, data: &[u8]) {
        self.write_to(&(data.len() as u32));
        self.write(data)
    }

    /// 写入字符串变长长度
    #[inline]
    pub fn write_str_bit7(&mut self, data: &str) {
        //self.bit7_write_u64(data.len() as u64);
        //self.write_ptr(data.as_ptr(),data.len());
        self.write_buff_bit7(data.as_bytes());
    }

    /// 写入字符串定长长度 u32 le
    #[inline]
    pub fn write_str_fixed_le(&mut self, data: &str) {
        self.write_buff_fixed_le(data.as_bytes())
    }

    /// 写入字符串定长长度 u32
    #[inline]
    pub fn write_str_fixed(&mut self, data: &str) {
        self.write_buff_fixed(data.as_bytes())
    }

    /// 读取二进制 变长长度
    #[inline]
    pub fn get_buff_bit7(&mut self) -> io::Result<(usize,Vec<u8>)> {
        let (size, len) = self.get_bit7::<u64>()?;
        Ok((size+len as usize,self.read(len as usize)?))
    }

    /// 读取字符串 变长
    #[inline]
    pub fn get_str_bit7(&mut self) -> io::Result<(usize,String)> {
        let (size,buff) = self.get_buff_bit7()?;
        Ok((size,String::from_utf8_lossy(&buff).to_string()))
    }

    /// 读取二进制 U32 LE
    #[inline]
    pub fn get_buff_fixed_le(&mut self) -> io::Result<Vec<u8>> {
        let len = self.get_le::<u32>()?;
        if len > 0 {
            return self.read(len as usize);
        }
        Ok(Vec::new())
    }
    /// 读取字符串 U32 LE
    #[inline]
    pub fn get_str_fixed_le(&mut self) -> io::Result<String> {
        let buff = self.get_buff_fixed_le()?;
        Ok(String::from_utf8_lossy(&buff).to_string())
    }
    /// 获取二进制 定长 u32
    #[inline]
    pub fn get_buff_fixed(&mut self) -> io::Result<Vec<u8>> {
        let len = self.get::<u32>()?;
        if len > 0 {
            return self.read(len as usize);
        }
        Ok(Vec::new())
    }

    /// 获取字符串 定长u32
    #[inline]
    pub fn get_str_fixed(&mut self) -> io::Result<String> {
        let buff = self.get_buff_fixed()?;
        Ok(String::from_utf8_lossy(&buff).to_string())
    }
}
#[inline]
fn zig_zag_encode_u16(v: i16) -> u16 {
    ((v << 1) ^ (v >> 15)) as u16
}
#[inline]
fn zig_zag_encode_u32(v: i32) -> u32 {
    ((v << 1) ^ (v >> 31)) as u32
}
#[inline]
fn zig_zag_encode_u64(v: i64) -> u64 {
    ((v << 1) ^ (v >> 63)) as u64
}
#[inline]
fn zig_zag_decode_i16(v: u16) -> i16 {
    ((v >> 1) as i16) ^ (-((v & 1) as i16))
}
#[inline]
fn zig_zag_decode_i32(v: u32) -> i32 {
    ((v >> 1) as i32) ^ (-((v & 1) as i32))
}
#[inline]
fn zig_zag_decode_i64(v: u64) -> i64 {
    ((v >> 1) as i64) ^ (-((v & 1) as i64))
}



pub auto trait Dummy{}
impl !Dummy for u8{}
impl Dummy for String{}
impl Dummy for Vec<u8>{}

