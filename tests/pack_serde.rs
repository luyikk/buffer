use anyhow::*;
use data_rw::{Data, DataReader, DataOwnedReader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[test]
pub fn test_pack_serde()->Result<()>{
    {
        let mut data = Data::new();
        data.pack_serialize(67i8)?;
        data.pack_serialize(66u8)?;
        data.pack_serialize(67i16)?;
        data.pack_serialize(66u16)?;
        data.pack_serialize(67i32)?;
        data.pack_serialize(66u32)?;
        data.pack_serialize(67i64)?;
        data.pack_serialize(66u64)?;
        data.pack_serialize(66.1111f32)?;
        data.pack_serialize(66.11112222f64)?;

        let mut data=DataReader::from(&data[..]);

        let v: i8 = data.pack_deserialize()?;
        assert_eq!(v, 67i8);
        let v: u8 = data.pack_deserialize()?;
        assert_eq!(v, 66u8);
        let v: i16 = data.pack_deserialize()?;
        assert_eq!(v, 67i16);
        let v: u16 = data.pack_deserialize()?;
        assert_eq!(v, 66u16);
        let v: i32 = data.pack_deserialize()?;
        assert_eq!(v, 67i32);
        let v: u32 = data.pack_deserialize()?;
        assert_eq!(v, 66u32);
        let v: i64 = data.pack_deserialize()?;
        assert_eq!(v, 67i64);
        let v: u64 = data.pack_deserialize()?;
        assert_eq!(v, 66u64);
        let v: f32 = data.pack_deserialize()?;
        assert_eq!(v, 66.1111f32);
        let v: f64 = data.pack_deserialize()?;
        assert_eq!(v, 66.11112222f64);
    }

    {
        let mut data = Data::new();

        data.pack_serialize("123123")?;
        data.pack_serialize(Some("123123".to_string()))?;
        data.pack_serialize(Some(66.11112222f64))?;
        let test = vec![1u8, 2u8, 3u8];
        data.pack_serialize(test)?;
        let test = vec![vec![1u8, 2u8, 3u8], vec![1u8, 2u8, 3u8]];
        data.pack_serialize(test)?;



        let mut data=DataReader::from(&data[..]);
        let v: String = data.pack_deserialize()?;
        assert_eq!(v, "123123");
        let v: Option<String> = data.pack_deserialize()?;
        assert_eq!(v, Some("123123".to_string()));
        let v: Option<f64> = data.pack_deserialize()?;
        assert_eq!(v, Some(66.11112222f64));
        let test: Vec<u8> = data.pack_deserialize()?;
        assert_eq!(test, vec![1u8, 2u8, 3u8]);
        let test: Vec<Vec<u8>> = data.pack_deserialize()?;
        assert_eq!(test, vec![vec![1u8, 2u8, 3u8], vec![1u8, 2u8, 3u8]]);


    }
    {
        let mut data = Data::new();

        let mut test1 = HashMap::new();
        test1.insert(1, 2);
        data.pack_serialize(Some(test1.clone()))?;

        let test2 = (1, 2, 3, "123123");
        data.pack_serialize(test2)?;

        #[derive(Serialize, Deserialize, PartialOrd, PartialEq, Debug)]
        pub struct LogOn {
            pub username: String,
            pub password: String
        }
        let test3 = LogOn { username: "123".into(), password: "321".into() };
        data.pack_serialize(&test3)?;


        let mut data=DataReader::from(&data[..]);

        let test: Option<HashMap<i32, i32>> = data.pack_deserialize()?;
        assert_eq!(Some(test1), test);

        let test: (i32, i32, i32, &str) = data.pack_deserialize()?;
        assert_eq!(test2, test);

        let test: LogOn = data.pack_deserialize()?;
        assert_eq!(test3,test);
    }

    Ok(())
}


#[test]
pub fn test_owned_pack_serde()->Result<()>{
    {
        let mut data = Data::new();
        data.pack_serialize(67i8)?;
        data.pack_serialize(66u8)?;
        data.pack_serialize(67i16)?;
        data.pack_serialize(66u16)?;
        data.pack_serialize(67i32)?;
        data.pack_serialize(66u32)?;
        data.pack_serialize(67i64)?;
        data.pack_serialize(66u64)?;
        data.pack_serialize(66.1111f32)?;
        data.pack_serialize(66.11112222f64)?;

        let mut data=DataOwnedReader::new(data.into());

        let v: i8 = data.pack_deserialize()?;
        assert_eq!(v, 67i8);
        let v: u8 = data.pack_deserialize()?;
        assert_eq!(v, 66u8);
        let v: i16 = data.pack_deserialize()?;
        assert_eq!(v, 67i16);
        let v: u16 = data.pack_deserialize()?;
        assert_eq!(v, 66u16);
        let v: i32 = data.pack_deserialize()?;
        assert_eq!(v, 67i32);
        let v: u32 = data.pack_deserialize()?;
        assert_eq!(v, 66u32);
        let v: i64 = data.pack_deserialize()?;
        assert_eq!(v, 67i64);
        let v: u64 = data.pack_deserialize()?;
        assert_eq!(v, 66u64);
        let v: f32 = data.pack_deserialize()?;
        assert_eq!(v, 66.1111f32);
        let v: f64 = data.pack_deserialize()?;
        assert_eq!(v, 66.11112222f64);
    }

    {
        let mut data = Data::new();

        data.pack_serialize("123123")?;
        data.pack_serialize(Some("123123".to_string()))?;
        data.pack_serialize(Some(66.11112222f64))?;
        let test = vec![1u8, 2u8, 3u8];
        data.pack_serialize(test)?;
        let test = vec![vec![1u8, 2u8, 3u8], vec![1u8, 2u8, 3u8]];
        data.pack_serialize(test)?;



        let mut data=DataOwnedReader::new(data.into());
        let v: String = data.pack_deserialize()?;
        assert_eq!(v, "123123");
        let v: Option<String> = data.pack_deserialize()?;
        assert_eq!(v, Some("123123".to_string()));
        let v: Option<f64> = data.pack_deserialize()?;
        assert_eq!(v, Some(66.11112222f64));
        let test: Vec<u8> = data.pack_deserialize()?;
        assert_eq!(test, vec![1u8, 2u8, 3u8]);
        let test: Vec<Vec<u8>> = data.pack_deserialize()?;
        assert_eq!(test, vec![vec![1u8, 2u8, 3u8], vec![1u8, 2u8, 3u8]]);


    }
    {
        let mut data = Data::new();

        let mut test1 = HashMap::new();
        test1.insert(1, 2);
        data.pack_serialize(Some(test1.clone()))?;

        let test2 = (1, 2, 3, "123123");
        data.pack_serialize(test2)?;

        #[derive(Serialize, Deserialize, PartialOrd, PartialEq, Debug)]
        pub struct LogOn {
            pub username: String,
            pub password: String
        }
        let test3 = LogOn { username: "123".into(), password: "321".into() };
        data.pack_serialize(&test3)?;


        let mut data=DataOwnedReader::new(data.into());

        let test: Option<HashMap<i32, i32>> = data.pack_deserialize()?;
        assert_eq!(Some(test1), test);

        let test: (i32, i32, i32, &str) = data.pack_deserialize()?;
        assert_eq!(test2, test);

        let test: LogOn = data.pack_deserialize()?;
        assert_eq!(test3,test);
    }

    Ok(())
}
