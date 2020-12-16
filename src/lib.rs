use bytes::buf::UninitSlice;
use bytes::{Buf, BufMut};
use paste::paste;
use std::io;
use std::io::{ErrorKind};
use std::collections::{HashMap, BTreeMap};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::collections::hash_map::RandomState;


#[derive(Debug)]
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

impl Data {
    #[inline]
    pub fn write_to_le<T: Writer>(&mut self, p: &T) {
        p.write_to_le(self)
    }
    #[inline]
    pub fn write_to<T: Writer>(&mut self, p: &T) {
        p.write_to(self)
    }
    #[inline]
    pub fn write_to_bit7<T:Writer>(&mut self,p:&T){
        p.write_to_bit7(self)
    }

}

pub trait Writer {
    fn write_to_le(&self, data: &mut Data);
    fn write_to(&self, data: &mut Data);
    fn write_to_bit7(&self,data:&mut Data);
}

macro_rules! make_writer {
    ($type:ty) => {
        impl Writer for $type {
            paste! {
             #[inline]
            fn write_to_le(&self, data:&mut Data) {
               data.[<put_ $type _le>](*self)
            }
             #[inline]
            fn write_to(&self, data: &mut Data) {
                data.[<put_ $type>](*self);
            }
             #[inline]
            fn write_to_bit7(&self, data: &mut Data) {
                data.[<bit7_write_ $type>](self)
            }
        }
      }
    };
}
impl Writer for bool {
    #[inline]
    fn write_to_le(&self, data: &mut Data) {
        if *self {
            data.put_u8(1);
        } else {
            data.put_u8(0);
        }
    }
    #[inline]
    fn write_to(&self, data: &mut Data) {
        if *self {
            data.put_u8(1);
        } else {
            data.put_u8(0);
        }
    }
    #[inline]
    fn write_to_bit7(&self, data: &mut Data) {
        if *self {
            data.put_u8(1);
        } else {
            data.put_u8(0);
        }
    }
}
impl Writer for u8 {
    #[inline]
    fn write_to_le(&self, data: &mut Data) {
        data.put_u8(*self);
    }
    #[inline]
    fn write_to(&self, data: &mut Data) {
        data.put_u8(*self);
    }
    #[inline]
    fn write_to_bit7(&self, data: &mut Data) {
        data.write_to_le(self)
    }
}
impl Writer for i8 {
    #[inline]
    fn write_to_le(&self, data: &mut Data) {
        data.put_i8(*self);
    }
    #[inline]
    fn write_to(&self, data: &mut Data) {
        data.put_i8(*self);
    }
    #[inline]
    fn write_to_bit7(&self, data: &mut Data) {
        data.write_to_le(self)
    }
}
make_writer!(i16);
make_writer!(i32);
make_writer!(i64);
make_writer!(u16);
make_writer!(u32);
make_writer!(u64);
make_writer!(u128);
make_writer!(i128);
make_writer!(f32);
make_writer!(f64);

impl Writer for String{
    #[inline]
    fn write_to_le(&self, data: &mut Data) {
       data.write_str_fixed_le(self);
    }
    #[inline]
    fn write_to(&self, data: &mut Data) {
       data.write_str_fixed(self);
    }
    #[inline]
    fn write_to_bit7(&self, data: &mut Data) {
        data.write_str_bit7(self)
    }
}
impl Writer for &str{
    #[inline]
    fn write_to_le(&self, data: &mut Data) {
        data.write_str_fixed_le(self);
    }
    #[inline]
    fn write_to(&self, data: &mut Data) {
        data.write_str_fixed(self);
    }
    #[inline]
    fn write_to_bit7(&self, data: &mut Data) {
        data.write_str_bit7(self)
    }
}
impl Writer for &[u8]{
    #[inline]
    fn write_to_le(&self, data: &mut Data) {
        data.write_buff_fixed_le(self)
    }
    #[inline]
    fn write_to(&self, data: &mut Data) {
        data.write_buff_fixed(self)
    }

    fn write_to_bit7(&self, data: &mut Data) {
        data.write_buff_bit7(self)
    }
}
impl<K:Writer,V:Writer> Writer for HashMap<K,V>{
    #[inline]
    fn write_to_le(&self, data: &mut Data) {
        data.put_u32_le(self.len() as u32);
        for (k,v) in self {
            data.write_to_le(k);
            data.write_to_le(v);
        }
    }
    #[inline]
    fn write_to(&self, data: &mut Data) {
        data.put_u32(self.len() as u32);
        for (k,v) in self {
            data.write_to(k);
            data.write_to(v);
        }
    }
    #[inline]
    fn write_to_bit7(&self, data: &mut Data) {
        data.write_to_bit7(&(self.len() as u64));
        for (k,v) in self {
            data.write_to_bit7(k);
            data.write_to_bit7(v);
        }
    }
}

impl <K:Writer,V:Writer> Writer for BTreeMap<K,V>{
    #[inline]
    fn write_to_le(&self, data: &mut Data) {
        data.put_u32_le(self.len() as u32);
        for (k,v) in self {
            data.write_to_le(k);
            data.write_to_le(v);
        }
    }
    #[inline]
    fn write_to(&self, data: &mut Data) {
        data.put_u32(self.len() as u32);
        for (k,v) in self {
            data.write_to(k);
            data.write_to(v);
        }
    }
    #[inline]
    fn write_to_bit7(&self, data: &mut Data) {
        data.write_to_bit7(&(self.len() as u64));
        for (k,v) in self {
            data.write_to_bit7(k);
            data.write_to_bit7(v);
        }
    }
}

impl <T:Writer> Writer for Vec<T>{
    #[inline]
    fn write_to_le(&self, data: &mut Data) {
        data.put_u32_le(self.len() as u32);
        for i in self {
            data.write_to_le(i);
        }
    }
    #[inline]
    fn write_to(&self, data: &mut Data) {
        data.put_u32(self.len() as u32);
        for i in self {
            data.write_to(i);
        }
    }
    #[inline]
    fn write_to_bit7(&self, data: &mut Data) {
        data.write_to_bit7(&(self.len() as u64));
        for i in self {
            data.write_to_bit7(i);
        }
    }
}

impl Data {
    #[inline]
    pub fn get<T: Reader>(&mut self) -> io::Result<T> {
        T::get(self)
    }
    #[inline]
    pub fn get_le<T: Reader>(&mut self) -> io::Result<T> {
        T::get_le(self)
    }

    #[inline]
    pub fn get_bit7<T:Reader>(&mut self)->io::Result<(usize,T)>{
        T::get_bit7(self)
    }
}

pub trait Reader {
    fn get(data: &mut Data) -> io::Result<Self> where Self: Sized;
    fn get_le(data: &mut Data) -> io::Result<Self> where Self: Sized;
    fn get_bit7(data:&mut Data)->io::Result<(usize,Self)> where Self:Sized;
}

macro_rules! make_reader {
    ($type:ty) => {
        impl Reader for $type {
            #[inline]
            fn get(data: &mut Data) ->  io::Result<Self> {
                if data.have_len() < std::mem::size_of::<$type>(){
                    return Err(io::Error::new(ErrorKind::InvalidData,format!("have len too small line:{}",line!())))
                }
                paste! {
                  Ok(data.[<get_ $type>]())
                }
            }
            #[inline]
            fn get_le(data: &mut Data) ->  io::Result<Self> {
                if data.have_len() < std::mem::size_of::<$type>(){
                    return Err(io::Error::new(ErrorKind::InvalidData,format!("have len too small line:{}",line!())))
                }
                paste! {
                    Ok(data.[<get_ $type _le>]())
                }
            }

            #[inline]
            fn get_bit7(data: &mut Data) -> io::Result<(usize, Self)> where Self: Sized {
                paste! {
                    data.[<read_bit7_ $type>]()
                }
            }
        }
    };
}

impl Reader for bool {
    #[inline]
    fn get(data: &mut Data) -> io::Result<Self> {
        if data.have_len() < std::mem::size_of::<u8>(){
            return Err(io::Error::new(ErrorKind::InvalidData,format!("have len too small line:{}",line!())))
        }
        if data.get_u8() == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    #[inline]
    fn get_le(data: &mut Data) -> io::Result<Self> {
        if data.have_len() < std::mem::size_of::<u8>(){
            return Err(io::Error::new(ErrorKind::InvalidData,format!("have len too small line:{}",line!())))
        }

        if data.get_u8() == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    #[inline]
    fn get_bit7(data: &mut Data) -> io::Result<(usize, Self)> where Self: Sized {
        Ok((1,data.get_le::<bool>()?))
    }
}
impl Reader for u8 {
    #[inline]
    fn get(data: &mut Data) ->  io::Result<Self> {
        if data.have_len() < std::mem::size_of::<u8>(){
            return Err(io::Error::new(ErrorKind::InvalidData,format!("have len too small line:{}",line!())))
        }

        Ok(data.get_u8())
    }
    #[inline]
    fn get_le(data: &mut Data) ->  io::Result<Self> {
        if data.have_len() < std::mem::size_of::<u8>(){
            return Err(io::Error::new(ErrorKind::InvalidData,format!("have len too small line:{}",line!())))
        }

        Ok(data.get_u8())
    }
    #[inline]
    fn get_bit7(data: &mut Data) -> io::Result<(usize, Self)> where Self: Sized {
        Ok((1,data.get_le::<u8>()?))
    }
}
impl Reader for i8 {
    #[inline]
    fn get(data: &mut Data) -> io::Result<Self> {
        if data.have_len() < std::mem::size_of::<i8>(){
            return Err(io::Error::new(ErrorKind::InvalidData,format!("have len too small line:{}",line!())))
        }
        Ok(data.get_i8())
    }
    #[inline]
    fn get_le(data: &mut Data) -> io::Result<Self> {
        if data.have_len() < std::mem::size_of::<i8>(){
            return Err(io::Error::new(ErrorKind::InvalidData,format!("have len too small line:{}",line!())))
        }
        Ok(data.get_i8())
    }
    #[inline]
    fn get_bit7(data: &mut Data) -> io::Result<(usize, Self)> where Self: Sized {
        Ok((1,data.get_le::<i8>()?))
    }
}
impl Reader for String{
    #[inline]
    fn get(data: &mut Data) -> io::Result<Self> where Self: Sized {
         data.get_str_fixed()
    }
    #[inline]
    fn get_le(data: &mut Data) -> io::Result<Self> where Self: Sized {
        data.get_str_fixed_le()
    }
    #[inline]
    fn get_bit7(data: &mut Data) -> io::Result<(usize, Self)> where Self: Sized {
        data.get_str_bit7()
    }
}
impl <T:Reader> Reader for Vec<T>{
    #[inline]
    fn get(data: &mut Data) -> io::Result<Self> where Self: Sized {
        let len = data.get::<u32>()? as usize;
        let mut v =Vec::with_capacity(len);
        for _ in 0..len {
            v.push(data.get::<T>()?);
        }
        Ok(v)
    }
    #[inline]
    fn get_le(data: &mut Data) -> io::Result<Self> where Self: Sized {
        let len = data.get_le::<u32>()? as usize;
        let mut v =Vec::with_capacity(len);
        for _ in 0..len {
            v.push(data.get_le::<T>()?);
        }
        Ok(v)
    }
    #[inline]
    fn get_bit7(data: &mut Data) -> io::Result<(usize, Self)> where Self: Sized {
        let (mut size,len)= data.get_bit7::<u64>()?;
        let mut vec =Vec::with_capacity(len as usize);
        for _ in 0..len{
            let (s,v)= data.get_bit7::<T>()?;
            vec.push(v);
            size+=s;
        }
        Ok((size,vec))
    }
}
impl <K:Reader+Eq+Hash,V:Reader> Reader for HashMap<K,V>{
    #[inline]
    fn get(data: &mut Data) -> io::Result<Self> where Self: Sized {
        let len=data.get::<u32>()? as usize;
        let mut map=HashMap::with_capacity(len);
        for _ in 0..len {
            map.insert(data.get::<K>()?,data.get::<V>()?);
        }

        Ok(map)
    }
    #[inline]
    fn get_le(data: &mut Data) -> io::Result<Self> where Self: Sized {
        let len=data.get_le::<u32>()? as usize;
        let mut map=HashMap::with_capacity(len);
        for _ in 0..len {
            map.insert(data.get_le::<K>()?,data.get_le::<V>()?);
        }

        Ok(map)
    }
    #[inline]
    fn get_bit7(data: &mut Data) -> io::Result<(usize, Self)> where Self: Sized {
        let (mut size,len)=data.get_bit7::<u64>()?;
        let mut map=HashMap::with_capacity(len as usize);
        for _ in 0..len {
            let (k_size,k)=data.get_bit7::<K>()?;
            let (v_size,v)=data.get_bit7::<V>()?;

            map.insert(k,v);
            size+=k_size;
            size+=v_size;
        }
        Ok((size,map))
    }
}
impl <K:Reader+Ord,V:Reader> Reader for BTreeMap<K,V>{
    #[inline]
    fn get(data: &mut Data) -> io::Result<Self> where Self: Sized {
        let len=data.get::<u32>()? as usize;
        let mut map=BTreeMap::new();
        for _ in 0..len {
            map.insert(data.get::<K>()?,data.get::<V>()?);
        }

        Ok(map)
    }
    #[inline]
    fn get_le(data: &mut Data) -> io::Result<Self> where Self: Sized {
        let len=data.get_le::<u32>()? as usize;
        let mut map=BTreeMap::new();
        for _ in 0..len {
            map.insert(data.get_le::<K>()?,data.get_le::<V>()?);
        }

        Ok(map)
    }
    #[inline]
    fn get_bit7(data: &mut Data) -> io::Result<(usize, Self)> where Self: Sized {
        let (mut size,len)=data.get_bit7::<u64>()?;

        let mut map=BTreeMap::new();
        for _ in 0..len {
            let (k_size,k)=data.get_bit7::<K>()?;
            let (v_size,v)=data.get_bit7::<V>()?;

            map.insert(k,v);
            size+=k_size;
            size+=v_size;
        }
        Ok((size,map))
    }
}

make_reader!(i16);
make_reader!(i32);
make_reader!(i64);
make_reader!(i128);
make_reader!(u16);
make_reader!(u32);
make_reader!(u64);
make_reader!(u128);
make_reader!(f32);
make_reader!(f64);




macro_rules! make_into {
    ($type:ty) => {
        impl Into<$type> for Data{
            #[inline]
            fn into(mut self) -> $type {
                let size=std::mem::size_of::<$type>();
                if self.len()<size{
                    panic!("data len < {}",size)
                }
                paste! {
                    self.[<get_ $type _le>]()
                }
            }
        }
    };
}

impl Into<u8> for Data{
    #[inline]
    fn into(mut self) -> u8 {
        if self.len()==0{
            panic!("data len < {}",std::mem::size_of::<u8>())
        }
        self.get_u8()
    }
}

impl Into<i8> for Data{
    #[inline]
    fn into(mut self) -> i8 {
        if self.len()==0{
            panic!("data len < {}",std::mem::size_of::<u8>())
        }
        self.get_i8()
    }
}

make_into!(i16);
make_into!(u16);
make_into!(i32);
make_into!(u32);
make_into!(i64);
make_into!(u64);
make_into!(i128);
make_into!(u128);
make_into!(f32);
make_into!(f64);



impl Into<String> for Data{
    #[inline]
    fn into(self) -> String {
        String::from_utf8_lossy(&self.buf).to_string()
    }
}


impl<T:Reader> Into<Vec<T>> for Data{
    #[inline]
    fn into(mut self) -> Vec<T> {
        let len= self.get_le::<i32>().expect("into vec len error:") as usize;
        let mut vec=Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(self.get_le::<T>().expect("read vec error:"))
        }
        vec
    }
}

impl <K:Reader+Eq+Hash,V:Reader> Into<HashMap<K,V>> for Data{
    #[inline]
    fn into(mut self) -> HashMap<K, V, RandomState> {
        let len= self.get_le::<i32>().expect("into hashmap len error:") as usize;
        let mut hashmap=HashMap::with_capacity(len);
        for _ in 0..len{
            hashmap.insert(self.get_le::<K>().expect("read hashmap  key error:"),
                           self.get_le::<V>().expect("read hashmap  value error:"));
        }
        hashmap
    }
}

impl <K:Reader+Ord,V:Reader>  Into<BTreeMap<K,V>> for Data{
    #[inline]
    fn into(mut self) -> BTreeMap<K, V> {
        let len= self.get_le::<i32>().expect("into BTreeMap len error:") as usize;
        let mut btreemap=BTreeMap::new();
        for _ in 0..len{
            btreemap.insert(self.get_le::<K>().expect("read BTreeMap  key error:"),
                           self.get_le::<V>().expect("read BTreeMap  value error:"));
        }
        btreemap
    }
}

pub trait ReadFrom{
    fn readfrom(data:&mut Data)->io::Result<Self> where Self: Sized;
}

macro_rules! make_read_from {
    ($type:ty) => {
        impl ReadFrom for $type{
             #[inline]
            fn readfrom(data: &mut Data) -> io::Result<Self> {
                data.set_position(0);
                <$type>::get_le(data)
            }
        }
    };
}

make_read_from!(i8);
make_read_from!(u8);
make_read_from!(i16);
make_read_from!(u16);
make_read_from!(i32);
make_read_from!(u32);
make_read_from!(i64);
make_read_from!(u64);
make_read_from!(i128);
make_read_from!(u128);
make_read_from!(f32);
make_read_from!(f64);



impl ReadFrom for String{
    #[inline]
    fn readfrom(data: &mut Data) -> io::Result<Self> where Self: Sized {
        data.set_position(0);
        Ok(String::from_utf8_lossy(&data.buf).to_string())
    }
}

impl<T:Reader> ReadFrom for Vec<T>{
    #[inline]
    fn readfrom(data: &mut Data) -> io::Result<Self> where Self: Sized {
        data.set_position(0);
        let len= data.get_le::<i32>()? as usize;
        let mut vec=Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(data.get_le::<T>()?);
        }
        Ok(vec)
    }
}

impl <K:Reader+Eq+Hash,V:Reader> ReadFrom for HashMap<K,V>{
    #[inline]
    fn readfrom(data: &mut Data) -> io::Result<Self> where Self: Sized {
        data.set_position(0);
        let len= data.get_le::<i32>()? as usize;
        let mut hashmap=HashMap::with_capacity(len);
        for _ in 0..len {
            hashmap.insert(data.get_le::<K>()?,data.get_le::<V>()?);
        }
        Ok(hashmap)
    }
}

impl <K:Reader+Ord,V:Reader> ReadFrom for BTreeMap<K,V>{
    #[inline]
    fn readfrom(data: &mut Data) -> io::Result<Self> where Self: Sized {
        data.set_position(0);
        let len= data.get_le::<i32>()? as usize;
        let mut btreemap=BTreeMap::new();
        for _ in 0..len{
            btreemap.insert(data.get_le::<K>()?,
                            data.get_le::<V>()?);
        }
        Ok(btreemap)
    }
}

pub trait ReadAs<T>{
    fn read_as(&mut self)->io::Result<T>;
}


impl <T:ReadFrom> ReadAs<T> for Data{
    #[inline]
    fn read_as(&mut self) -> io::Result<T> {
        T::readfrom(self)
    }
}