use crate::{Data, Reader, Dummy};
use bytes::Buf;
use std::hash::Hash;
use std::collections::{HashMap, BTreeMap};
use std::collections::hash_map::RandomState;
use paste::paste;
use crate::readfrom::ReadFrom;

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



impl<T:ReadFrom+Reader+Dummy> Into<Vec<T>> for Data{
    fn into(mut self) -> Vec<T> {
        self.set_position(0);
        let len= self.get_le::<i32>().expect("into vec len error:") as usize;
        let mut vec=Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::get_le(&mut self).expect("into vec len error:"));
        }
        vec
    }
}

impl Into<Vec<u8>> for Data{
    fn into(self) -> Vec<u8> {
        self.buf
    }
}

impl Into<String> for Data{
    fn into(self) -> String {
        String::from_utf8_lossy(&self.buf).into()
    }
}

impl <K:Reader+Eq+Hash,V:Reader> Into<HashMap<K,V>> for Data{
    #[inline]
    fn into(mut self) -> HashMap<K, V, RandomState> {
        self.set_position(0);
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
        self.set_position(0);
        let len= self.get_le::<i32>().expect("into BTreeMap len error:") as usize;
        let mut btreemap=BTreeMap::new();
        for _ in 0..len{
            btreemap.insert(self.get_le::<K>().expect("read BTreeMap  key error:"),
                            self.get_le::<V>().expect("read BTreeMap  value error:"));
        }
        btreemap
    }
}