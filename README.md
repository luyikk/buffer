#Fast binary serialization read write,support Serde and Stream model
## Similar to bincode, but beyond the performance of bincode
## Provide basic RW and var length Rw

```rust
let mut data = Data::new();
data.write_fixed(1u8);
data.write_fixed(2i16);
data.write_fixed(3i32);
data.write_fixed(4i64);
data.write_fixed(5f32);
data.write_fixed(6f64);
data.write_fixed("hello world");
data.write_var_integer(123u64);
data.write_var_integer("hello world");

let mut rd = DataReader::from(&data[..]);
assert_eq!(1, rd.read_fixed::<u8>()?);
assert_eq!(2, rd.read_fixed::<i16>()?);
assert_eq!(3, rd.read_fixed::<i32>()?);
assert_eq!(4, rd.read_fixed::<i64>()?);
assert_eq!(5f32, rd.read_fixed::<f32>()?);
assert_eq!(6f64, rd.read_fixed::<f64>()?);
assert_eq!("hello world", rd.read_fixed_str()?);
assert_eq!(123u64, rd.read_var_integer::<u64>()?);
assert_eq!("hello world", rd.read_var_str()?);
```


benchmark
```shell
running 9 tests
test bench_base_rw       ... bench:      52,288 ns/iter (+/- 654)     data-rw
test bench_owned_base_rw ... bench:      53,659 ns/iter (+/- 2,795)   data-rw
test bench_owned_pack    ... bench:     140,562 ns/iter (+/- 1,925)
test bench_pack          ... bench:     139,910 ns/iter (+/- 2,353)
test bench_str           ... bench:      53,023 ns/iter (+/- 770)     data-rw
test bench_str_bincode   ... bench:      71,520 ns/iter (+/- 1,558)
test bench_str_json      ... bench:      91,993 ns/iter (+/- 1,074)
test bench_str_owned     ... bench:      53,272 ns/iter (+/- 1,277)   data-rw
test bench_str_rmp       ... bench:      78,455 ns/iter (+/- 1,333)

test result: ok. 0 passed; 0 failed; 0 ignored; 9 measured; 0 filtered out; finished in 2.24s
```


