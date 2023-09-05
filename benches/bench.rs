use data_rw;
use data_rw::{DataOwnedReader, DataReader};
use serde::{Deserialize, Serialize};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

#[inline]
fn bench_owned_pack(size: usize) {
    for _ in 0..size {
        let mut data = data_rw::Data::new();
        data.pack_serialize(67i32).unwrap();
        data.pack_serialize("123123").unwrap();
        let mut data = DataOwnedReader::new(data.into());
        let v: i32 = data.pack_deserialize().unwrap();
        assert_eq!(v, 67i32);
        let v: &str = data.pack_deserialize().unwrap();
        assert_eq!(v, "123123");
    }
}

#[inline]
fn bench_pack(size: usize) {
    for _ in 0..size {
        let mut data = data_rw::Data::new();
        data.pack_serialize(67i32).unwrap();
        data.pack_serialize("123123").unwrap();
        let mut data = DataReader::from(&data[..]);
        let v: i32 = data.pack_deserialize().unwrap();
        assert_eq!(v, 67i32);
        let v: &str = data.pack_deserialize().unwrap();
        assert_eq!(v, "123123");
    }
}

#[inline]
fn bench_str(size: usize) {
    for _ in 0..size {
        let mut data = data_rw::Data::new();
        data.serde_serialize("123123").unwrap();
        let mut data = DataReader::from(&data);
        let v: &str = data.serde_deserialize().unwrap();
        assert_eq!(v, "123123");
    }
}

#[inline]
fn bench_str_owned(size: usize) {
    for _ in 0..size {
        let mut data = data_rw::Data::new();
        data.serde_serialize("123123").unwrap();
        let mut data = DataOwnedReader::new(data.into());
        let v: &str = data.serde_deserialize().unwrap();
        assert_eq!(v, "123123");
    }
}

#[inline]
fn bench_str_rmp(size: usize) {
    for _ in 0..size {
        let buff = rmp_serde::encode::to_vec("123123").unwrap();
        let v: &str = rmp_serde::decode::from_slice(&buff).unwrap();
        assert_eq!(v, "123123");
    }
}

#[inline]
fn bench_str_json(size: usize) {
    for _ in 0..size {
        let buff = serde_json::to_vec("123123").unwrap();
        let v: &str = serde_json::from_slice(&buff).unwrap();
        assert_eq!(v, "123123");
    }
}

#[inline]
fn bench_str_bincode(size: usize) {
    for _ in 0..size {
        let buff = bincode::serialize("123123").unwrap();
        let v: &str = bincode::deserialize(&buff).unwrap();
        assert_eq!(v, "123123");
    }
}

#[inline]
fn bench_base_rw(size: usize) {
    for _ in 0..size {
        let mut data = data_rw::Data::new();
        data.write_fixed(67i32);
        data.write_fixed("123123");
        let mut data = DataReader::from(&data[..]);
        let v: i32 = data.read_fixed().unwrap();
        assert_eq!(v, 67i32);
        let v = data.read_fixed_str().unwrap();
        assert_eq!(v, "123123");
    }
}

#[inline]
fn bench_owned_base_rw(size: usize) {
    for _ in 0..size {
        let mut data = data_rw::Data::new();
        data.write_fixed(67i32);
        data.write_fixed("123123");
        let mut data = DataOwnedReader::new(data.into());
        let v: i32 = data.read_fixed().unwrap();
        assert_eq!(v, 67i32);
        let v = data.read_fixed_str().unwrap();
        assert_eq!(v, "123123");
    }
}

#[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
struct Foo {
    a: i32,
    b: String,
    c: u128,
    d: f64,
}

#[inline]
fn bench_foo(size: usize) {
    let test = Foo {
        a: 123,
        b: "test".to_string(),
        c: 333,
        d: 0.25,
    };
    for _ in 0..size {
        let data = data_rw::Data::serialize(&test).unwrap();
        let mut data = DataReader::from(&data);
        let v: Foo = data.serde_deserialize().unwrap();
        assert_eq!(v, test);
    }
}
#[inline]
fn bench_foo_owned(size: usize) {
    let test = Foo {
        a: 123,
        b: "test".to_string(),
        c: 333,
        d: 0.25,
    };
    for _ in 0..size {
        let mut data = data_rw::Data::new();
        data.serde_serialize(&test).unwrap();
        let mut data = DataOwnedReader::new(data.into());
        let v: Foo = data.serde_deserialize().unwrap();
        assert_eq!(v, test);
    }
}

#[inline]
fn bench_foo_rmp(size: usize) {
    let test = Foo {
        a: 123,
        b: "test".to_string(),
        c: 333,
        d: 0.25,
    };

    for _ in 0..size {
        let buff = rmp_serde::encode::to_vec(&test).unwrap();
        let v: Foo = rmp_serde::decode::from_slice(&buff).unwrap();
        assert_eq!(v, test);
    }
}

#[inline]
fn bench_foo_json(size: usize) {
    let test = Foo {
        a: 123,
        b: "test".to_string(),
        c: 333,
        d: 0.25,
    };
    for _ in 0..size {
        let buff = serde_json::to_vec(&test).unwrap();
        let v: Foo = serde_json::from_slice(&buff).unwrap();
        assert_eq!(v, test);
    }
}

#[inline]
fn bench_foo_bincode(size: usize) {
    let test = Foo {
        a: 123,
        b: "test".to_string(),
        c: 333,
        d: 0.25,
    };
    for _ in 0..size {
        let buff = bincode::serialize(&test).unwrap();
        let v: Foo = bincode::deserialize(&buff).unwrap();
        assert_eq!(v, test);
    }
}

fn benchmark(c: &mut Criterion) {
    let size: usize = 1000;
    c.bench_with_input(BenchmarkId::new("bench_base_rw", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.iter(|| bench_base_rw(s));
    });
    c.bench_with_input(
        BenchmarkId::new("bench_owned_base_rw", size),
        &size,
        |b, &s| {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.iter(|| bench_owned_base_rw(s));
        },
    );
    c.bench_with_input(
        BenchmarkId::new("bench_owned_pack", size),
        &size,
        |b, &s| {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.iter(|| bench_owned_pack(s));
        },
    );

    c.bench_with_input(BenchmarkId::new("bench_pack", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.iter(|| bench_pack(s));
    });

    c.bench_with_input(BenchmarkId::new("bench_str", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.iter(|| bench_str(s));
    });

    c.bench_with_input(BenchmarkId::new("bench_str_owned", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.iter(|| bench_str_owned(s));
    });
    c.bench_with_input(BenchmarkId::new("bench_str_rmp", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.iter(|| bench_str_rmp(s));
    });
    c.bench_with_input(BenchmarkId::new("bench_str_json", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.iter(|| bench_str_json(s));
    });
    c.bench_with_input(
        BenchmarkId::new("bench_str_bincode", size),
        &size,
        |b, &s| {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.iter(|| bench_str_bincode(s));
        },
    );

    c.bench_with_input(BenchmarkId::new("bench_foo", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.iter(|| bench_foo(s));
    });
    c.bench_with_input(BenchmarkId::new("bench_foo_owned", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.iter(|| bench_foo_owned(s));
    });
    c.bench_with_input(BenchmarkId::new("bench_foo_rmp", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.iter(|| bench_foo_rmp(s));
    });
    c.bench_with_input(BenchmarkId::new("bench_foo_json", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.iter(|| bench_foo_json(s));
    });
    c.bench_with_input(
        BenchmarkId::new("bench_foo_bincode", size),
        &size,
        |b, &s| {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.iter(|| bench_foo_bincode(s));
        },
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
