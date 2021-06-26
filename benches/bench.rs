#![feature(test)]
extern crate test;

use test::Bencher;
use data_rw::*;


#[bench]
fn bench_owned_pack(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..1000 {
            let mut data = data_rw::Data::new();
            data.pack_serialize(67i32).unwrap();
            data.pack_serialize("123123").unwrap();
            let mut data=DataOwnedReader::new(data.into());
            let v: i32 = data.pack_deserialize().unwrap();
            assert_eq!(v, 67i32);
            let v: &str = data.pack_deserialize().unwrap();
            assert_eq!(v, "123123");
        }
    });
}


#[bench]
fn bench_pack(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..1000 {
            let mut data = data_rw::Data::new();
            data.pack_serialize(67i32).unwrap();
            data.pack_serialize("123123").unwrap();
            let mut data=DataReader::from(&data[..]);
            let v: i32 = data.pack_deserialize().unwrap();
            assert_eq!(v, 67i32);
            let v: &str = data.pack_deserialize().unwrap();
            assert_eq!(v, "123123");
        }
    });
}


#[bench]
fn bench_str(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..1000 {
            let mut data = data_rw::Data::new();
            data.serde_serialize("123123").unwrap();
            let mut data=DataReader::from(&data);
            let v:&str = data.serde_deserialize().unwrap();
            assert_eq!(v, "123123");
        }
    });
}
#[bench]
fn bench_str_owned(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..1000 {
            let mut data = data_rw::Data::new();
            data.serde_serialize("123123").unwrap();
            let mut data=DataOwnedReader::new(data.into());
            let v: &str= data.serde_deserialize().unwrap();
            assert_eq!(v, "123123");
        }
    });
}

#[bench]
fn bench_str_rmp(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..1000 {
            let buff = rmp_serde::encode::to_vec("123123").unwrap();
            let v: &str=  rmp_serde::decode::from_read_ref(&buff).unwrap();
            assert_eq!(v, "123123");
        }
    });
}

#[bench]
fn bench_str_json(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..1000 {
            let buff =  serde_json::to_vec("123123").unwrap();
            let v: &str=  serde_json::from_slice(&buff).unwrap();
            assert_eq!(v, "123123");
        }
    });
}

#[bench]
fn bench_str_bincode(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..1000 {
            let buff= bincode::serialize("123123").unwrap();
            let v:&str= bincode::deserialize(&buff).unwrap();
            assert_eq!(v, "123123");
        }
    });
}



#[bench]
fn bench_base_rw(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..1000 {
            let mut data = data_rw::Data::new();
            data.write_fixed(67i32);
            data.write_fixed("123123");
            let mut data = DataReader::from(&data[..]);
            let v: i32 = data.read_fixed().unwrap();
            assert_eq!(v, 67i32);
            let v = data.read_fixed_str().unwrap();
            assert_eq!(v, "123123");
        }
    });
}

#[bench]
fn bench_owned_base_rw(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..1000 {
            let mut data = data_rw::Data::new();
            data.write_fixed(67i32);
            data.write_fixed("123123");
            let mut data = DataOwnedReader::new(data.into());
            let v: i32 = data.read_fixed().unwrap();
            assert_eq!(v, 67i32);
            let v = data.read_fixed_str().unwrap();
            assert_eq!(v, "123123");
        }
    });
}