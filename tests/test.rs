use data_rw::{Data, ReadAs};
use std::error::Error;
use std::collections::{HashMap, BTreeMap};
use bytes::Buf;

#[test]
fn test_le()->Result<(),Box<dyn Error>> {
    let mut data = Data::new();
    data.write_to_le(&34u8);
    let v = data.get_le::<u8>()?;
    assert_eq!(v, 34u8);

    data.write_to_le(&4);
    let v = data.get_le::<i32>()?;
    assert_eq!(v, 4);

    data.write_to_le(&true);
    let v = data.get_le::<bool>()?;
    assert_eq!(v, true);

    data.write_to_le(&0.556f32);
    let v = data.get_le::<f32>()?;
    assert_eq!(v, 0.556f32);

    data.write_to_le(&"adfadfaf");
    let v = data.get_le::<String>()?;
    assert_eq!(v, "adfadfaf");

    let vec:Vec<i32>=vec![2,3,4,5,6,7,7];
    data.write_to_le(&vec);

    let v= data.get_le::<Vec<i32>>()?;
    assert_eq!(v, vec);

    let vec=vec!["11","22","33","44"];
    data.write_to_le(&vec);
    let v= data.get_le::<Vec<String>>()?;
    assert_eq!(v, vec);

    let mut hashmap=HashMap::new();
    hashmap.insert(1,"123123".to_string());
    hashmap.insert(2,"123123".to_string());
    data.write_to_le(&hashmap);

    let v=data.get_le::<HashMap<i32,String>>()?;
    assert_eq!(v, hashmap);


    let mut btreemap=BTreeMap::new();
    btreemap.insert(1,"123123".to_string());
    btreemap.insert(2,"123123".to_string());
    data.write_to_le(&hashmap);

    let v=data.get_le::<BTreeMap<i32,String>>()?;
    assert_eq!(v, btreemap);

    Ok(())
}

#[test]
fn test()->Result<(),Box<dyn Error>> {
    let mut data = Data::new();

    data.write_to_le(&34u8);
    let v = data.get_le::<u8>()?;
    assert_eq!(v, 34u8);

    data.write_to(&4);
    let v = data.get::<i32>()?;
    assert_eq!(v, 4);

    data.write_to(&true);
    let v = data.get::<bool>()?;
    assert_eq!(v, true);

    data.write_to(&0.556f32);
    let v = data.get::<f32>()?;
    assert_eq!(v, 0.556f32);

    data.write_to(&"adfadfaf");
    let v = data.get::<String>()?;
    assert_eq!(v, "adfadfaf");

    let vec:Vec<i32>=vec![2,3,4,5,6,7,7];
    data.write_to(&vec);

    let v= data.get::<Vec<i32>>()?;
    assert_eq!(v, vec);

    let vec=vec!["11","22","33","44"];
    data.write_to(&vec);
    let v= data.get::<Vec<String>>()?;
    assert_eq!(v, vec);

    let mut hashmap=HashMap::new();
    hashmap.insert(1,"123123".to_string());
    hashmap.insert(2,"123123".to_string());
    data.write_to(&hashmap);

    let v=data.get::<HashMap<i32,String>>()?;
    assert_eq!(v, hashmap);


    let mut btreemap=BTreeMap::new();
    btreemap.insert(1,"123123".to_string());
    btreemap.insert(2,"123123".to_string());
    data.write_to(&hashmap);

    let v=data.get::<BTreeMap<i32,String>>()?;
    assert_eq!(v, btreemap);

    Ok(())
}

#[test]
fn test_bit7()->Result<(),Box<dyn Error>> {
    let mut data = Data::new();

    data.write_to_bit7(&34u8);
    let (_,v) = data.get_bit7::<u8>()?;
    assert_eq!(v, 34u8);

    data.write_to_bit7(&4);
    let (_,v) = data.get_bit7::<i32>()?;
    assert_eq!(v, 4);

    data.write_to_bit7(&true);
    let (_,v) = data.get_bit7::<bool>()?;
    assert_eq!(v, true);

    data.write_to_bit7(&0.556f32);
    let (_,v) = data.get_bit7::<f32>()?;
    assert_eq!(v, 0.556f32);

    data.write_to_bit7(&"adfadfaf");
    let (_,v) = data.get_bit7::<String>()?;
    assert_eq!(v, "adfadfaf");

    let vec:Vec<i32>=vec![2,3,4,5,6,7,7];
    data.write_to_bit7(&vec);

    let  (_,v)= data.get_bit7::<Vec<i32>>()?;
    assert_eq!(v, vec);

    let vec=vec!["11","22","33","44"];
    data.write_to_bit7(&vec);
    let  (_,v)= data.get_bit7::<Vec<String>>()?;
    assert_eq!(v, vec);

    let mut hashmap=HashMap::new();
    hashmap.insert(1,"123123".to_string());
    hashmap.insert(2,"123123".to_string());
    data.write_to_bit7(&hashmap);

    let (_,v)=data.get_bit7::<HashMap<i32,String>>()?;
    assert_eq!(v, hashmap);


    let mut btreemap=BTreeMap::new();
    btreemap.insert(1,"123123".to_string());
    btreemap.insert(2,"123123".to_string());
    data.write_to_bit7(&hashmap);

    let (_,v)=data.get_bit7::<BTreeMap<i32,String>>()?;
    assert_eq!(v, btreemap);

    Ok(())
}

#[test]
fn test_deref_mut()->Result<(),Box<dyn Error>> {
    let mut data = Data::new();
    data.write_to_le(&34u8);
    data.write_to_le(&4);
    data.write_to_le(&true);
    data.write_to_le(&0.556f32);
    data.write_to_le(&"adfadfaf");
    let vec:Vec<i32>=vec![2,3,4,5,6,7,7];
    data.write_to_le(&vec);
    let vec=vec!["11","22","33","44"];
    data.write_to_le(&vec);

    let buff=data.bytes();

    println!("{:?}",buff);

    fn copy(source:&[u8],target:&mut [u8]){
        target.copy_from_slice(source);
    }

    let mut data=Data::with_len(buff.len(),0);
    copy(buff,&mut data);

    let v = data.get_le::<u8>()?;
    assert_eq!(v, 34u8);
    let v = data.get_le::<i32>()?;
    assert_eq!(v, 4);
    let v = data.get_le::<bool>()?;
    assert_eq!(v, true);
    let v = data.get_le::<f32>()?;
    assert_eq!(v, 0.556f32);
    let v = data.get_le::<String>()?;
    assert_eq!(v, "adfadfaf");
    let vec:Vec<i32>=vec![2,3,4,5,6,7,7];
    let v= data.get_le::<Vec<i32>>()?;
    assert_eq!(v, vec);
    let vec=vec!["11","22","33","44"];
    let v= data.get_le::<Vec<String>>()?;
    assert_eq!(v, vec);

    Ok(())

}

#[test]
fn test_into(){

    let mut data = Data::new();
    let vec=vec![1u8,2u8,3u8,4u8];
    data.write_to_le(&vec);
    let vec2:Vec<u8>=data.into();
    assert_eq!(vec2[4..], vec);

    let mut data = Data::new();
    let vec=vec!["11","22","33","44"];
    data.write_to_le(&vec);
    let vec2:Vec<String>=data.into();
    assert_eq!(vec2, vec);

    let mut data = Data::new();
    let mut hashmap=HashMap::new();
    hashmap.insert(1,"123123".to_string());
    hashmap.insert(2,"123123".to_string());
    data.write_to_le(&hashmap);

    let hashmap2:HashMap<i32,String>=data.into();
    assert_eq!(hashmap, hashmap2);
}

#[test]
fn test_read_as()->Result<(),Box<dyn Error>>{

    let mut data = Data::new();
    let vec=vec![1u8,2u8,3u8,4u8];
    data.write_to_le(&vec);
    let vec2:Vec<u8>=data.read_as()?;
    assert_eq!(vec2[4..], vec);

    let mut data = Data::new();
    let vec=vec![vec];
    data.write_to_le(&vec);
    let vec2:Vec<Vec<u8>>=data.read_as()?;
    assert_eq!(vec2, vec);


    let mut data = Data::new();
    let vec=vec!["11","22","33","44"];
    data.write_to_le(&vec);
    let vec2:Vec<String>=data.read_as()?;
    assert_eq!(vec2, vec);

    let mut data = Data::new();
    let mut hashmap=HashMap::new();
    hashmap.insert(1,"123123".to_string());
    hashmap.insert(2,"123123".to_string());
    data.write_to_le(&hashmap);

    let hashmap2:HashMap<i32,String>=data.read_as()?;
    assert_eq!(hashmap, hashmap2);

    Ok(())
}
