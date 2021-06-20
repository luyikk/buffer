#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn current(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..10000 {
            let mut data = data_rw::Data::new();
            data.pack_serialize(&67i32).unwrap();
            let v: i32 = data.pack_deserialize().unwrap();
            assert_eq!(v, 67i32);
        }
    });
}

#[bench]
fn rmp(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..10000 {
            let mut data = data_rw::Data::new();

            let buff = rmp_serde::encode::to_vec(&67i32).unwrap();
            data.write_buff_fixed_le(&buff);


            let len = data.get_le::<u32>().unwrap() as usize;
            let start = data.get_position();
            if !data.set_position(start + len) {
                panic!("index overflow {}", line!())
            }
            let v: i32 = rmp_serde::decode::from_read_ref(&data[start..data.get_position()]).unwrap();
            assert_eq!(v, 67i32);
        }
    });
}

#[bench]
fn json(b: &mut Bencher){
    b.iter(move ||{
        for _ in 0..10000 {
            let mut data = data_rw::Data::new();

            let buff = serde_json::to_vec(&67i32).unwrap();
            data.write_buff_fixed_le(&buff);


            let len = data.get_le::<u32>().unwrap() as usize;
            let start = data.get_position();
            if !data.set_position(start + len) {
                panic!("index overflow {}", line!())
            }
            let v: i32 = serde_json::from_slice(&data[start..data.get_position()]).unwrap();
            assert_eq!(v, 67i32);
        }
    });
}