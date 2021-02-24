use crate::{Data, Reader, Dummy};
use std::io;
use std::hash::Hash;
use std::collections::{HashMap, BTreeMap};

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



impl<T:Reader+Dummy> ReadFrom for Vec<T>{
    fn readfrom(data: &mut Data) -> io::Result<Self> where Self: Sized {
        data.set_position(0);
        let len= data.get_le::<i32>()? as usize;
        let mut vec=Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::get_le(data)?);
        }
        Ok(vec)
    }
}

impl ReadFrom for Vec<u8>{
    fn readfrom(data: &mut Data) -> io::Result<Self> where Self: Sized {
        Ok(data.buf.clone())
    }
}


impl ReadFrom for Data{
    fn readfrom(data: &mut Data) -> io::Result<Self> where Self: Sized {
        data.set_position(0);
        Ok(data.clone())
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
