use crate::{Data, Writer, Dummy};
use std::collections::{HashMap, BTreeMap};

pub trait ToData:Sized{
    fn to_data(self) -> Data;
}

macro_rules! make_into_data {
    ($type:ty) => {
        impl ToData for $type{
            #[inline]
            fn to_data(self) -> Data {
                let mut data=Data::with_capacity(16);
                data.write_to_le(&self);
                data
            }
        }
    };
}

make_into_data!(i8);
make_into_data!(u8);
make_into_data!(i16);
make_into_data!(u16);
make_into_data!(i32);
make_into_data!(u32);
make_into_data!(i64);
make_into_data!(u64);
make_into_data!(i128);
make_into_data!(u128);
make_into_data!(f32);
make_into_data!(f64);

impl ToData for String{
    #[inline]
    fn to_data(self) -> Data {
        let mut data=Data::new();
        data.write(self.as_bytes());
        data
    }
}

impl ToData for &str{
    #[inline]
    fn to_data(self) -> Data {
        let mut data=Data::new();
        data.write(self.as_bytes());
        data
    }
}

impl <T:Writer+Dummy> ToData for Vec<T>{
    #[inline]
    fn to_data(self) -> Data {
        let mut buff = Data::with_capacity(1024);
        buff.write_to_le(&self);
        buff
    }
}

impl ToData for Vec<u8>{
    #[inline]
    fn to_data(self) -> Data {
        let mut data=Data::with_capacity(self.len()+4);
        data.write(&self);
        data
    }
}

impl<K:Writer,V:Writer> ToData for HashMap<K,V>{
    fn to_data(self) -> Data {
        let mut buff = Data::with_capacity(1024);
        buff.write_to_le(&self);
        buff

    }
}

impl<K:Writer,V:Writer> ToData for BTreeMap<K,V>{
    fn to_data(self) -> Data {
        let mut buff = Data::with_capacity(1024);
        buff.write_to_le(&self);
        buff
    }
}