use anyhow::Result;
use data_rw::data_owned_reader::DataOwnedReader;
use data_rw::{Data, DataReader};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[test]
pub fn test_serde_ser() -> Result<()> {
    {
        let mut data = Data::new();
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
        data.serde_serialize(6.0f32)?;
        data.serde_serialize(7.0f64)?;

        let mut rdata = DataReader::from(&data[..]);

        assert_eq!(true, rdata.read_fixed::<bool>()?);
        assert_eq!(false, rdata.read_fixed::<bool>()?);
        assert_eq!(1, rdata.read_fixed::<i8>()?);
        assert_eq!(1, rdata.read_fixed::<u8>()?);
        assert_eq!(2, rdata.read_fixed::<i16>()?);
        assert_eq!(2, rdata.read_fixed::<u16>()?);
        assert_eq!(3, rdata.read_fixed::<i32>()?);
        assert_eq!(3, rdata.read_fixed::<u32>()?);
        assert_eq!(4, rdata.read_fixed::<i64>()?);
        assert_eq!(4, rdata.read_fixed::<u64>()?);
        assert_eq!(6.0f32, rdata.read_fixed::<f32>()?);
        assert_eq!(7.0f64, rdata.read_fixed::<f64>()?);

        let mut rdata = DataReader::from(&data[..]);

        assert_eq!(true, rdata.serde_deserialize::<bool>()?);
        assert_eq!(false, rdata.serde_deserialize::<bool>()?);
        assert_eq!(1, rdata.serde_deserialize::<i8>()?);
        assert_eq!(1, rdata.serde_deserialize::<u8>()?);
        assert_eq!(2, rdata.serde_deserialize::<i16>()?);
        assert_eq!(2, rdata.serde_deserialize::<u16>()?);
        assert_eq!(3, rdata.serde_deserialize::<i32>()?);
        assert_eq!(3, rdata.serde_deserialize::<u32>()?);
        assert_eq!(4, rdata.serde_deserialize::<i64>()?);
        assert_eq!(4, rdata.serde_deserialize::<u64>()?);
        assert_eq!(6.0f32, rdata.serde_deserialize::<f32>()?);
        assert_eq!(7.0f64, rdata.serde_deserialize::<f64>()?);
    }

    {
        let mut data = Data::new();
        data.serde_serialize("123123")?;
        data.serde_serialize("123123".to_string())?;
        data.serde_serialize(b"123123")?;
        data.serde_serialize(b"123123".to_vec())?;
        let c = "你".to_string().chars().next().unwrap();
        data.serde_serialize(c)?;
        data.serde_serialize("123123".as_bytes().to_vec())?;

        {
            let mut rdata = DataReader::from(&data[..]);
            assert_eq!("123123", rdata.read_fixed_str()?);
            assert_eq!("123123", rdata.read_fixed_str()?);
            assert_eq!("123123", rdata.read_fixed_str()?);
            assert_eq!("123123".as_bytes(), rdata.read_fixed_buf()?);
            assert_eq!("你", rdata.read_fixed_str()?);
            assert_eq!("123123".as_bytes(), rdata.read_fixed_buf()?.to_vec());
        }
        {
            let mut rdata = DataReader::from(&data[..]);
            assert_eq!("123123", rdata.serde_deserialize::<&str>()?);
            assert_eq!("123123", rdata.serde_deserialize::<String>()?);
            assert_eq!("123123".as_bytes(), rdata.serde_deserialize::<&[u8]>()?);
            assert_eq!("123123".as_bytes(), rdata.serde_deserialize::<Vec<u8>>()?);
            assert_eq!("你", rdata.serde_deserialize::<&str>()?);
            assert_eq!(b"123123", rdata.serde_deserialize::<&[u8]>()?);
        }
    }

    {
        let mut data = Data::new();
        let mut map = HashMap::new();
        map.insert("123123".to_string(), 1);
        data.serde_serialize(map.clone())?;

        let mut map2 = BTreeMap::new();
        map2.insert("123123".to_string(), 1);
        data.serde_serialize(map2.clone())?;

        let mut rdata = DataReader::from(&data[..]);
        assert_eq!(map, rdata.serde_deserialize::<HashMap<String, i32>>()?);
        assert_eq!(map2, rdata.serde_deserialize::<BTreeMap<String, i32>>()?);
    }

    Ok(())
}

#[test]
pub fn test_serde_de() -> Result<()> {
    {
        let mut data = Data::new();
        let x = Some(100i32);
        data.serde_serialize(x)?;
        let x1: Option<i32> = None;
        data.serde_serialize(x1)?;
        data.serde_serialize(())?;

        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
        struct Foo;
        data.serde_serialize(Foo)?;

        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
        struct Foo2(u8);
        data.serde_serialize(Foo2(100))?;

        let mut data = DataReader::from(&data[..]);
        assert_eq!(x, data.serde_deserialize::<Option<i32>>()?);
        assert_eq!(None, data.serde_deserialize::<Option<i32>>()?);
        assert_eq!((), data.serde_deserialize::<()>()?);
        assert_eq!(Foo, data.serde_deserialize::<Foo>()?);
        assert_eq!(Foo2(100), data.serde_deserialize::<Foo2>()?);
    }

    {
        let mut data = Data::new();
        let a1 = (1, 2, "123".to_string(), 0.5f32);
        data.serde_serialize(a1.clone())?;

        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        struct Foo(i32, i32, String, f32);
        let a2 = Foo(1, 2, "123".to_string(), 0.5f32);
        data.serde_serialize(a2.clone())?;

        let mut data = DataReader::from(&data[..]);
        assert_eq!(a1, data.serde_deserialize::<(i32, i32, String, f32)>()?);
        assert_eq!(a2, data.serde_deserialize::<Foo>()?);
    }
    {
        let mut data = Data::new();
        let mut map = BTreeMap::new();
        map.insert("1".to_string(), 1);
        map.insert("2".to_string(), 2);
        map.insert("3".to_string(), 3);
        data.serde_serialize(map.clone())?;

        let mut bmap = BTreeMap::new();
        bmap.insert("1".to_string(), 1);
        bmap.insert("2".to_string(), 2);
        bmap.insert("3".to_string(), 3);
        data.serde_serialize(bmap.clone())?;

        let mut data = DataReader::from(&data[..]);
        assert_eq!(map, data.serde_deserialize::<BTreeMap<String, i32>>()?);
        assert_eq!(bmap, data.serde_deserialize::<BTreeMap<String, i32>>()?);
    }
    {
        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        struct Rgb {
            r: u8,
            g: u8,
            b: u8,
        }

        let a = Rgb {
            r: 200,
            g: 244,
            b: 100,
        };
        let mut data = Data::new();
        data.serde_serialize(a.clone())?;

        let mut data = DataReader::from(&data[..]);
        assert_eq!(a, data.serde_deserialize::<Rgb>()?);
    }
    {
        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        enum E<'a> {
            T(u8, u8),
            U(String, u32, u32),
            S(&'a str, u32, u32),
        }
        let mut xdata = Data::new();
        xdata.serde_serialize(E::T(44, 66))?;
        xdata.serde_serialize(E::U("123123".to_string(), 44, 66))?;
        xdata.serde_serialize(E::S("321321", 44, 66))?;

        let mut data = DataReader::from(&xdata[..]);

        let b = data.serde_deserialize::<E>()?;
        assert_eq!(E::T(44, 66), b);

        let b = data.serde_deserialize::<E>()?;
        assert_eq!(E::U("123123".to_string(), 44, 66), b);

        let b = data.serde_deserialize::<E>()?;
        assert_eq!(E::S("321321", 44, 66), b);
    }
    {
        let mut data = Data::new();
        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        enum E {
            S { r: u8, g: u8, b: u8 },
            P { r: u8, g: u8, b: u8, a: u8 },
        }

        data.serde_serialize(E::S {
            r: 255,
            g: 244,
            b: 105,
        })?;
        data.serde_serialize(E::P {
            r: 255,
            g: 244,
            b: 105,
            a: 11,
        })?;

        let mut data = DataReader::from(&data[..]);

        let b = data.serde_deserialize::<E>()?;
        assert_eq!(
            E::S {
                r: 255,
                g: 244,
                b: 105
            },
            b
        );
        let b = data.serde_deserialize::<E>()?;
        assert_eq!(
            E::P {
                r: 255,
                g: 244,
                b: 105,
                a: 11
            },
            b
        );
    }
    {
        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        enum E {
            S { r: u8, g: u8, b: u8 },
            P { r: u8, g: u8, b: u8, a: u8 },
        }

        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        struct Foo {
            a: Option<E>,
            b: String,
            c: f64,
            d: (i32, f64),
            e: Vec<Vec<u8>>,
        }

        let test = Foo {
            a: Some(E::P {
                r: 1,
                g: 2,
                b: 3,
                a: 4,
            }),
            b: "test".to_string(),
            c: 0.555679f64,
            d: (12, 0.555679f64),
            e: vec![vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]],
        };

        let mut data = Data::new();
        data.serde_serialize(test.clone())?;
        let mut data = DataReader::from(&data[..]);
        assert_eq!(test, data.serde_deserialize::<Foo>()?)
    }

    Ok(())
}

#[test]
pub fn test_owned_serde_ser() -> Result<()> {
    {
        let mut data = Data::new();
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
        data.serde_serialize(6.0f32)?;
        data.serde_serialize(7.0f64)?;

        let mut rdata = DataOwnedReader::new(data.into());

        assert_eq!(true, rdata.read_fixed::<bool>()?);
        assert_eq!(false, rdata.read_fixed::<bool>()?);
        assert_eq!(1, rdata.read_fixed::<i8>()?);
        assert_eq!(1, rdata.read_fixed::<u8>()?);
        assert_eq!(2, rdata.read_fixed::<i16>()?);
        assert_eq!(2, rdata.read_fixed::<u16>()?);
        assert_eq!(3, rdata.read_fixed::<i32>()?);
        assert_eq!(3, rdata.read_fixed::<u32>()?);
        assert_eq!(4, rdata.read_fixed::<i64>()?);
        assert_eq!(4, rdata.read_fixed::<u64>()?);
        assert_eq!(6.0f32, rdata.read_fixed::<f32>()?);
        assert_eq!(7.0f64, rdata.read_fixed::<f64>()?);

        rdata.set_offset(0)?;

        assert_eq!(true, rdata.serde_deserialize::<bool>()?);
        assert_eq!(false, rdata.serde_deserialize::<bool>()?);
        assert_eq!(1, rdata.serde_deserialize::<i8>()?);
        assert_eq!(1, rdata.serde_deserialize::<u8>()?);
        assert_eq!(2, rdata.serde_deserialize::<i16>()?);
        assert_eq!(2, rdata.serde_deserialize::<u16>()?);
        assert_eq!(3, rdata.serde_deserialize::<i32>()?);
        assert_eq!(3, rdata.serde_deserialize::<u32>()?);
        assert_eq!(4, rdata.serde_deserialize::<i64>()?);
        assert_eq!(4, rdata.serde_deserialize::<u64>()?);
        assert_eq!(6.0f32, rdata.serde_deserialize::<f32>()?);
        assert_eq!(7.0f64, rdata.serde_deserialize::<f64>()?);
    }

    {
        let mut data = Data::new();
        data.serde_serialize("123123")?;
        data.serde_serialize("123123".to_string())?;
        data.serde_serialize(b"123123")?;
        data.serde_serialize(b"123123".to_vec())?;
        let c = "你".to_string().chars().next().unwrap();
        data.serde_serialize(c)?;
        data.serde_serialize("123123".as_bytes().to_vec())?;

        {
            let mut rdata = DataOwnedReader::new(data.clone().into());
            assert_eq!("123123", rdata.read_fixed_str()?);
            assert_eq!("123123", rdata.read_fixed_str()?);
            assert_eq!("123123", rdata.read_fixed_str()?);
            assert_eq!("123123".as_bytes(), rdata.read_fixed_buf()?);
            assert_eq!("你", rdata.read_fixed_str()?);
            assert_eq!("123123".as_bytes(), rdata.read_fixed_buf()?.to_vec());
        }
        {
            let mut rdata = DataOwnedReader::new(data.into());
            assert_eq!("123123", rdata.serde_deserialize::<&str>()?);
            assert_eq!("123123", rdata.serde_deserialize::<String>()?);
            assert_eq!("123123".as_bytes(), rdata.serde_deserialize::<&[u8]>()?);
            assert_eq!("123123".as_bytes(), rdata.serde_deserialize::<Vec<u8>>()?);
            assert_eq!("你", rdata.serde_deserialize::<&str>()?);
            assert_eq!(b"123123", rdata.serde_deserialize::<&[u8]>()?);
        }
    }

    {
        let mut data = Data::new();
        let mut map = HashMap::new();
        map.insert("123123".to_string(), 1);
        data.serde_serialize(map.clone())?;

        let mut map2 = BTreeMap::new();
        map2.insert("123123".to_string(), 1);
        data.serde_serialize(map2.clone())?;

        let mut rdata = DataOwnedReader::new(data.into());
        assert_eq!(map, rdata.serde_deserialize::<HashMap<String, i32>>()?);
        assert_eq!(map2, rdata.serde_deserialize::<BTreeMap<String, i32>>()?);
    }

    Ok(())
}

#[test]
pub fn test_owned_serde_de() -> Result<()> {
    {
        let mut data = Data::new();
        let x = Some(100i32);
        data.serde_serialize(x)?;
        let x1: Option<i32> = None;
        data.serde_serialize(x1)?;
        data.serde_serialize(())?;

        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
        struct Foo;
        data.serde_serialize(Foo)?;

        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
        struct Foo2(u8);
        data.serde_serialize(Foo2(100))?;

        let mut data = DataOwnedReader::new(data.into());
        assert_eq!(x, data.serde_deserialize::<Option<i32>>()?);
        assert_eq!(None, data.serde_deserialize::<Option<i32>>()?);
        assert_eq!((), data.serde_deserialize::<()>()?);
        assert_eq!(Foo, data.serde_deserialize::<Foo>()?);
        assert_eq!(Foo2(100), data.serde_deserialize::<Foo2>()?);
    }

    {
        let mut data = Data::new();
        let a1 = (1, 2, "123".to_string(), 0.5f32);
        data.serde_serialize(a1.clone())?;

        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        struct Foo(i32, i32, String, f32);
        let a2 = Foo(1, 2, "123".to_string(), 0.5f32);
        data.serde_serialize(a2.clone())?;

        let mut data = DataOwnedReader::new(data.into());
        assert_eq!(a1, data.serde_deserialize::<(i32, i32, String, f32)>()?);
        assert_eq!(a2, data.serde_deserialize::<Foo>()?);
    }
    {
        let mut data = Data::new();
        let mut map = BTreeMap::new();
        map.insert("1".to_string(), 1);
        map.insert("2".to_string(), 2);
        map.insert("3".to_string(), 3);
        data.serde_serialize(map.clone())?;

        let mut bmap = BTreeMap::new();
        bmap.insert("1".to_string(), 1);
        bmap.insert("2".to_string(), 2);
        bmap.insert("3".to_string(), 3);
        data.serde_serialize(bmap.clone())?;

        let mut data = DataOwnedReader::new(data.into());
        assert_eq!(map, data.serde_deserialize::<BTreeMap<String, i32>>()?);
        assert_eq!(bmap, data.serde_deserialize::<BTreeMap<String, i32>>()?);
    }
    {
        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        struct Rgb {
            r: u8,
            g: u8,
            b: u8,
        }

        let a = Rgb {
            r: 200,
            g: 244,
            b: 100,
        };
        let mut data = Data::new();
        data.serde_serialize(a.clone())?;

        let mut data = DataOwnedReader::new(data.into());
        assert_eq!(a, data.serde_deserialize::<Rgb>()?);
    }
    {
        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        enum E<'a> {
            T(u8, u8),
            U(String, u32, u32),
            S(&'a str, u32, u32),
        }
        let mut xdata = Data::new();
        xdata.serde_serialize(E::T(44, 66))?;
        xdata.serde_serialize(E::U("123123".to_string(), 44, 66))?;
        xdata.serde_serialize(E::S("321321", 44, 66))?;

        let mut data = DataOwnedReader::new(xdata.into());

        let b = data.serde_deserialize::<E>()?;
        assert_eq!(E::T(44, 66), b);

        let b = data.serde_deserialize::<E>()?;
        assert_eq!(E::U("123123".to_string(), 44, 66), b);

        let b = data.serde_deserialize::<E>()?;
        assert_eq!(E::S("321321", 44, 66), b);
    }
    {
        let mut data = Data::new();
        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        enum E {
            S { r: u8, g: u8, b: u8 },
            P { r: u8, g: u8, b: u8, a: u8 },
        }

        data.serde_serialize(E::S {
            r: 255,
            g: 244,
            b: 105,
        })?;
        data.serde_serialize(E::P {
            r: 255,
            g: 244,
            b: 105,
            a: 11,
        })?;

        let mut data = DataOwnedReader::new(data.into());

        let b = data.serde_deserialize::<E>()?;
        assert_eq!(
            E::S {
                r: 255,
                g: 244,
                b: 105
            },
            b
        );
        let b = data.serde_deserialize::<E>()?;
        assert_eq!(
            E::P {
                r: 255,
                g: 244,
                b: 105,
                a: 11
            },
            b
        );
    }
    {
        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        enum E {
            S { r: u8, g: u8, b: u8 },
            P { r: u8, g: u8, b: u8, a: u8 },
        }

        #[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
        struct Foo {
            a: Option<E>,
            b: String,
            c: f64,
            d: (i32, f64),
            e: Vec<Vec<u8>>,
        }

        let test = Foo {
            a: Some(E::P {
                r: 1,
                g: 2,
                b: 3,
                a: 4,
            }),
            b: "test".to_string(),
            c: 0.555679f64,
            d: (12, 0.555679f64),
            e: vec![vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]],
        };

        let mut data = Data::new();
        data.serde_serialize(test.clone())?;

        let mut data = DataOwnedReader::new(data.into());
        assert_eq!(test, data.serde_deserialize::<Foo>()?)
    }

    Ok(())
}
