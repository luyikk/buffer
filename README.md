#IO buffer write reader, support bit7 and normal mode, contain error check.


## Examples 

```rust
  
  let mut data = Data::new();
   
  data.write_to(&"adfadfaf");
  let v = data.get::<String>()?;
  assert_eq!(v, "adfadfaf");  
 
  // le
  data.write_to_le(&34u8);
  let v = data.get_le::<u8>()?;
  assert_eq!(v, 34u8);

  //bit7  
  let mut hashmap=HashMap::new();
  hashmap.insert(1,"123123".to_string());
  hashmap.insert(2,"123123".to_string());
  data.write_to_bit7(&hashmap);

  let (_,v)=data.get_bit7::<HashMap<i32,String>>()?;
  assert_eq!(v, hashmap);

```