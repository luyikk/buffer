use data_rw::{Data, ReadAs};
use std::error::Error;
use std::collections::{HashMap, BTreeMap};
use bytes::{Buf};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;


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


#[test]
pub fn test_serde_ser()->Result<(),Box<dyn Error>>{
    let mut data=Data::new();
    {
        data.serde_serialize(true)?;
        data.serde_serialize(false)?;
        data.serde_serialize(1i8)?;
        data.serde_serialize(1u8)?;
        data.serde_serialize(2i16)?;
        data.serde_serialize(2u16)?;
        data.serde_serialize(3i32)?;
        data.serde_serialize(3u32)?;
        data.serde_serialize(4i64)?;
        data.serde_serialize(4u64)?;
        // data.serde_serialize(5i128)?;
        // data.serde_serialize(5u128)?;
        data.serde_serialize(6.0f32)?;
        data.serde_serialize(7.0f64)?;

        assert_eq!(true, data.get_le::<bool>()?);
        assert_eq!(false, data.get_le::<bool>()?);
        assert_eq!(1, data.get_le::<i8>()?);
        assert_eq!(1, data.get_le::<u8>()?);
        assert_eq!(2, data.get_le::<i16>()?);
        assert_eq!(2, data.get_le::<u16>()?);
        assert_eq!(3, data.get_le::<i32>()?);
        assert_eq!(3, data.get_le::<u32>()?);
        assert_eq!(4, data.get_le::<i64>()?);
        assert_eq!(4, data.get_le::<u64>()?);
        assert_eq!(6.0f32, data.get_le::<f32>()?);
        assert_eq!(7.0f64, data.get_le::<f64>()?);
    }

    {
        data.serde_serialize("123123")?;
        assert_eq!("123123", data.get_le::<String>()?);

        data.serde_serialize("123123".to_string())?;
        assert_eq!("123123", data.get_le::<String>()?);

        data.serde_serialize("123123".as_bytes())?;
        assert_eq!("123123", data.get_le::<String>()?);

        data.serde_serialize("123123".as_bytes())?;
        assert_eq!("123123".as_bytes(), data.get_le::<Vec<u8>>()?);

        let c = "你".to_string().chars().next().unwrap();
        data.serde_serialize(c)?;
        assert_eq!("你", data.get_le::<String>()?);

        data.serde_serialize("123123".as_bytes().to_vec())?;
        assert_eq!("123123".as_bytes(), data.get_le::<Vec<u8>>()?);
    }

    {
        let mut map=HashMap::new();
        map.insert("123123".to_string(),1);

        data.serde_serialize(map.clone())?;
        assert_eq!(map, data.get_le::<HashMap<String,i32>>()?);

        let mut map=BTreeMap::new();
        map.insert("123123".to_string(),1);

        data.serde_serialize(map.clone())?;
        assert_eq!(map, data.get_le::<BTreeMap<String,i32>>()?);
    }

    #[derive(Deserialize,Serialize)]
    pub struct  Test{
        a:String,
        b:i32
    }


    let mut data=Data::new();
    data.serde_serialize(Test{a:"123123".to_string(),b:3333})?;
    println!("{:?}",data);

    Ok(())
}

#[test]
pub fn test_serde_de()->Result<(),Box<dyn Error>>{
    let mut data=Data::new();
    {
        data.serde_serialize(true)?;
        data.serde_serialize(false)?;
        data.serde_serialize(1i8)?;
        data.serde_serialize(1u8)?;
        data.serde_serialize(2i16)?;
        data.serde_serialize(2u16)?;
        data.serde_serialize(3i32)?;
        data.serde_serialize(3u32)?;
        data.serde_serialize(4i64)?;
        data.serde_serialize(4u64)?;
        // data.serde_serialize(5i128)?;
        // data.serde_serialize(5u128)?;
        data.serde_serialize(6.0f32)?;
        data.serde_serialize(7.0f64)?;

        assert_eq!(true, data.serde_deserialize::<bool>()?);
        assert_eq!(false, data.serde_deserialize::<bool>()?);
        assert_eq!(1i8, data.serde_deserialize::<i8>()?);
        assert_eq!(1u8, data.serde_deserialize::<u8>()?);
        assert_eq!(2i16, data.serde_deserialize::<i16>()?);
        assert_eq!(2u16, data.serde_deserialize::<u16>()?);
        assert_eq!(3i32, data.serde_deserialize::<i32>()?);
        assert_eq!(3u32, data.serde_deserialize::<u32>()?);
        assert_eq!(4i64, data.serde_deserialize::<i64>()?);
        assert_eq!(4u64, data.serde_deserialize::<u64>()?);
        assert_eq!(6.0f32, data.serde_deserialize::<f32>()?);
        assert_eq!(7.0f64, data.serde_deserialize::<f64>()?);
    }
    {

        data.serde_serialize("123123")?;
        let x=data.serde_deserialize::<String>()?;
        assert_eq!("123123", x);

        let c = "你".to_string().chars().next().unwrap();
        data.serde_serialize(c)?;
        assert_eq!(c, data.serde_deserialize::<char>()?);

        data.serde_serialize("123123".as_bytes().to_vec())?;
        assert_eq!("123123".as_bytes(), data.serde_deserialize::<Vec<u8>>()?);

        data.serde_serialize("123123".as_bytes().to_vec())?;
        assert_eq!("123123".as_bytes().to_vec(),  data.serde_deserialize::<ByteBuf>()?.into_vec());

    }
    {
        let x=Some(100i32);
        data.serde_serialize(x)?;
        assert_eq!(x, data.serde_deserialize::<Option<i32>>()?);
        let y:Option<i32>=None;
        data.serde_serialize(y)?;
        assert_eq!(None, data.serde_deserialize::<Option<i32>>()?);

        data.serde_serialize(())?;
        assert_eq!((), data.serde_deserialize::<()>()?);

        #[derive(Deserialize,Serialize,Debug,PartialOrd, PartialEq)]
        struct Foo;
        let x=Foo;
        data.serde_serialize(x)?;
        let y= data.serde_deserialize::<Foo>()?;
        assert_eq!(Foo,y);

        #[derive(Deserialize,Serialize,Debug,PartialOrd, PartialEq)]
        struct Foo2(u8);
        let x=Foo2(100);
        data.serde_serialize(x)?;
        let y= data.serde_deserialize::<Foo2>()?;
        assert_eq!(Foo2(100),y)

    }

    {
        let a=(1,2,"123".to_string(),0.5f32);
        data.serde_serialize(a.clone())?;
        let b= data.serde_deserialize::<(i32,i32,String,f32)>()?;
        assert_eq!(a,b);

        #[derive(Deserialize,Serialize,Debug,PartialOrd, PartialEq,Clone)]
        struct Foo(i32,i32,String,f32);
        let a=Foo(1,2,"123".to_string(),0.5f32);
        data.serde_serialize(a.clone())?;
        let b= data.serde_deserialize::<Foo>()?;
        assert_eq!(a,b);
    }
    {
        let mut map=BTreeMap::new();
        map.insert("1".to_string(),1);
        map.insert("2".to_string(),2);
        map.insert("3".to_string(),3);
        data.serde_serialize(map.clone())?;
        let a= data.serde_deserialize::<BTreeMap<String,i32>>()?;
        assert_eq!(map,a);

        let mut bmap=BTreeMap::new();
        bmap.insert("1".to_string(),1);
        bmap.insert("2".to_string(),2);
        bmap.insert("3".to_string(),3);
        data.serde_serialize(bmap.clone())?;
        let b= data.serde_deserialize::<BTreeMap<String,i32>>()?;
        assert_eq!(bmap,b);
    }
    {
        #[derive(Deserialize,Serialize,Debug,PartialOrd, PartialEq,Clone)]
        struct Rgb {
            r: u8,
            g: u8,
            b: u8,
        }

        let a=Rgb{
            r:200,
            g:244,
            b:100
        };

        data.serde_serialize(a.clone())?;
        let b= data.serde_deserialize::<Rgb>()?;
        assert_eq!(a,b);
    }
    {
        #[derive(Deserialize,Serialize,Debug,PartialOrd, PartialEq,Clone)]
        enum E {
            T(u8, u8),
            U(String, u32, u32),
        }
        let mut xdata=Data::new();
        xdata.serde_serialize(E::T(44,66))?;
        let b= xdata.serde_deserialize::<E>()?;
        assert_eq!(E::T(44,66),b);

        data.serde_serialize(E::U("123123".to_string(),44,66))?;
        let b= data.serde_deserialize::<E>()?;
        assert_eq!(E::U("123123".to_string(),44,66),b);
    }
    {
        #[derive(Deserialize,Serialize,Debug,PartialOrd, PartialEq,Clone)]
        enum E {
            S { r: u8, g: u8, b: u8 },
            P { r: u8, g: u8, b: u8,a:u8}
        }

        data.serde_serialize(E::S{r:255,g:244,b:105})?;
        let b= data.serde_deserialize::<E>()?;
        assert_eq!(E::S{r:255,g:244,b:105},b);

        data.serde_serialize(E::P{r:255,g:244,b:105,a:11})?;
        let b= data.serde_deserialize::<E>()?;
        assert_eq!(E::P{r:255,g:244,b:105,a:11},b);
    }
    {
        #[derive(Deserialize,Serialize,Debug,PartialOrd, PartialEq,Clone)]
        enum E {
            S { r: u8, g: u8, b: u8 },
            P { r: u8, g: u8, b: u8,a:u8}
        }

        #[derive(Deserialize,Serialize,Debug,PartialOrd, PartialEq,Clone)]
        struct Foo{
            a:Option<E>,
            b:String,
            c:f64,
            d:(i32,f64),
            e:Vec<Vec<u8>>
        }


        let test=Foo{
            a:Some(E::P {r:1,g:2,b:3,a:4}),
            b:"test".to_string(),
            c:0.555679f64,
            d:(12,0.555679f64),
            e:vec![vec![1,2,3,4,5],vec![5,4,3,2,1]]
        };

        let mut data=Data::new();
        data.serde_serialize(test.clone())?;
        println!("{:?}",data);
        assert_eq!(test,data.serde_deserialize::<Foo>()?)
    }

    Ok(())
}

#[test]
pub fn test_make()->Result<(),Box<dyn Error>>{

    let mut data=Data::new();
    data.write_to_le(&"123333");
    data.set_position(0);

    let len=data.len() +4;
    let mut buff=Data::with_capacity(len);
    buff.write_to_le(&(len as u32));
    buff.write(&data);

    println!("{:?}",buff);
    Ok(())

}

#[test]
pub fn test_struct_2()->Result<(),Box<dyn Error>>{
    #[derive(Serialize,Deserialize,PartialOrd, PartialEq,Debug)]
    pub enum Flag{
        Message(String),
        Int(i32)
    }

    #[derive(Serialize,Deserialize,PartialOrd, PartialEq,Debug)]
    pub struct LogOnResult{
        pub success:bool,
        pub msg:Flag
    }

    let mut data=Data::new();
    data.serde_serialize(LogOnResult{
        success: true,
        msg: Flag::Message("LogOn Ok".into())
    })?;

    let res= data.serde_deserialize::<LogOnResult>()?;

    assert_eq!(res,LogOnResult{
        success: true,
        msg: Flag::Message("LogOn Ok".into())
    });

    Ok(())
}

#[test]
pub fn test_msgpack_to_from()->Result<(),Box<dyn Error>>{
    {
        let mut data = Data::pack_from::<(bool, &str, i32, Option<String>)>((true, "ok", 1, None))?;
        assert_eq!((true, "ok", 1, None), data.pack_to::<(bool, &str, i32, Option<String>)>()?);
    }
    {
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
        struct Foo {
            name: String,
            arg: i32
        }

        let mut data = Data::pack_from(Foo {
            name: "1".to_string(),
            arg: 2
        })?;
        println!("{:?}", data);
        assert_eq!(Foo {
            name: "1".to_string(),
            arg: 2
        }, data.pack_to()?);
    }

    #[derive(Serialize, Deserialize, PartialOrd, PartialEq, Debug)]
    pub struct LogOnResult {
        #[serde(rename = "Success")]
        pub success: bool,
        #[serde(rename = "Msg")]
        pub msg: String,
    }

    let data = Data::pack_from(LogOnResult{
        success:true,
        msg:"1 ok".to_string()
    })?;

    println!("{:?}",data);


    let mut data = Data::pack_from(67i8)?;
    let v:i8=data.pack_to()?;
    assert_eq!(v,67i8);

    let mut data = Data::pack_from(66u8)?;
    let v:u8=data.pack_to()?;
    assert_eq!(v,66u8);

    let mut data = Data::pack_from(67i16)?;
    let v:i16=data.pack_to()?;
    assert_eq!(v,67i16);

    let mut data = Data::pack_from(66u16)?;
    let v:u16=data.pack_to()?;
    assert_eq!(v,66u16);

    let mut data = Data::pack_from(67i32)?;
    let v:i32=data.pack_to()?;
    assert_eq!(v,67i32);

    let mut data = Data::pack_from(66u32)?;
    let v:u32=data.pack_to()?;
    assert_eq!(v,66u32);

    let mut data = Data::pack_from(67i64)?;
    let v:i64=data.pack_to()?;
    assert_eq!(v,67i64);

    let mut data = Data::pack_from(66u64)?;
    let v:u64=data.pack_to()?;
    assert_eq!(v,66u64);

    let mut data = Data::pack_from("123123")?;
    let v:&str=data.pack_to()?;
    assert_eq!(v,"123123");

    let test=vec![1u8,2u8,3u8];
    let mut data = Data::pack_from(test)?;
    let test:Vec<u8>=data.pack_to()?;
    assert_eq!(test,vec![1u8,2u8,3u8]);

    let test=vec![vec![1u8,2u8,3u8],vec![1u8,2u8,3u8]];
    let mut data = Data::pack_from(test)?;
    let test:Vec<Vec<u8>>=data.pack_to()?;
    assert_eq!(test,vec![vec![1u8,2u8,3u8],vec![1u8,2u8,3u8]]);

    let mut test =HashMap::new();
    test.insert(1,2);
    let mut data = Data::pack_from(test.clone())?;
    let test2:HashMap<i32,i32>=data.pack_to()?;
    assert_eq!(test,test2);

    let mut test=(1,2,3,"123123");
    let mut data = Data::pack_from(test)?;
    test=data.pack_to()?;
    assert_eq!(test,(1,2,3,"123123"));

    Ok(())
}

#[test]
pub fn test_msgpack_serde()->Result<(),Box<dyn Error>>{

    let mut data =  Data::new();
    data.pack_serialize(67i8)?;
    let v:i8=data.pack_deserialize()?;
    assert_eq!(v,67i8);

    data.pack_serialize(66u8)?;
    let v:u8=data.pack_deserialize()?;
    assert_eq!(v,66u8);

    data.pack_serialize(67i16)?;
    let v:i16=data.pack_deserialize()?;
    assert_eq!(v,67i16);

    data.pack_serialize(66u16)?;
    let v:u16=data.pack_deserialize()?;
    assert_eq!(v,66u16);

    data.pack_serialize(67i32)?;
    let v:i32=data.pack_deserialize()?;
    assert_eq!(v,67i32);

    data.pack_serialize(66u32)?;
    let v:u32=data.pack_deserialize()?;
    assert_eq!(v,66u32);

    data.pack_serialize(67i64)?;
    let v:i64=data.pack_deserialize()?;
    assert_eq!(v,67i64);

    data.pack_serialize(66u64)?;
    let v:u64=data.pack_deserialize()?;
    assert_eq!(v,66u64);

    data.pack_serialize(66.1111f32)?;
    let v:f32=data.pack_deserialize()?;
    assert_eq!(v,66.1111f32);

    data.pack_serialize(66.11112222f64)?;
    let v:f64=data.pack_deserialize()?;
    assert_eq!(v,66.11112222f64);

    data.pack_serialize("123123")?;
    let v:String=data.pack_deserialize()?;
    assert_eq!(v,"123123");

    data.pack_serialize(Some("123123".to_string()))?;
    let v:Option<String>=data.pack_deserialize()?;
    assert_eq!(v,Some("123123".to_string()));

    data.pack_serialize(Some(66.11112222f64))?;
    let v:Option<f64>=data.pack_deserialize()?;
    assert_eq!(v,Some(66.11112222f64));

    let test=vec![1u8,2u8,3u8];
    data.pack_serialize(test)?;
    let test:Vec<u8>=data.pack_deserialize()?;
    assert_eq!(test,vec![1u8,2u8,3u8]);

    let test=vec![vec![1u8,2u8,3u8],vec![1u8,2u8,3u8]];
    data.pack_serialize(test)?;
    let test:Vec<Vec<u8>>=data.pack_deserialize()?;
    assert_eq!(test,vec![vec![1u8,2u8,3u8],vec![1u8,2u8,3u8]]);

    let mut test =HashMap::new();
    test.insert(1,2);
    data.pack_serialize(Some(test.clone()))?;
    let test2:Option<HashMap<i32,i32>>=data.pack_deserialize()?;
    assert_eq!(Some(test),test2);

    let test=(1,2,3,"123123");
    data.pack_serialize(test)?;
    let test2:(i32,i32,i32,String)=data.pack_deserialize()?;
    assert_eq!(test2,(1,2,3,"123123".to_string()));

    #[derive(Serialize,Deserialize,PartialOrd, PartialEq,Debug)]
    pub struct LogOn{
        pub username:String,
        pub password:String
    }

    let test=LogOn{ username:"123".into(),password:"321".into()};
    data.pack_serialize(test)?;
    let test:LogOn=data.pack_deserialize()?;
    assert_eq!(test,LogOn{ username:"123".into(),password:"321".into()});

    Ok(())
}

