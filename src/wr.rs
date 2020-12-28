use crate::Data;
use bytes::{BufMut, Buf};
use std::collections::{HashMap, BTreeMap};
use std::io;
use std::io::ErrorKind;
use std::hash::Hash;
use paste::paste;

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
