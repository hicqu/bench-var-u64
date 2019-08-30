#![feature(test)]
extern crate test;
use test::{black_box, Bencher};
use tikv_util::codec::number::{self, NumberEncoder};

#[bench]
fn bench_parse_u8(b: &mut Bencher) {
    let mut s = Vec::with_capacity(10);
    s.encode_u16(b'a' as u16).unwrap();
    s.encode_u16(b'b' as u16).unwrap();
    s.encode_u16(b'c' as u16).unwrap();
    b.iter(|| {
        for _ in 0..black_box(1000) {
            let mut ss = s.as_ref();
            assert_eq!(b'a' as u16, number::decode_u16(&mut ss).unwrap());
            assert_eq!(b'b' as u16, number::decode_u16(&mut ss).unwrap());
            assert_eq!(b'c' as u16, number::decode_u16(&mut ss).unwrap());
        }
    })
}

#[bench]
fn bench_parse_var64(b: &mut Bencher) {
    let mut s = Vec::with_capacity(3);
    s.encode_var_u64(b'a' as u64).unwrap();
    s.encode_var_u64(b'b' as u64).unwrap();
    s.encode_var_u64(b'c' as u64).unwrap();
    b.iter(|| {
        for _ in 0..black_box(1000) {
            let mut ss = s.as_ref();
            assert_eq!(b'a' as u64, number::decode_var_u64(&mut ss).unwrap());
            assert_eq!(b'b' as u64, number::decode_var_u64(&mut ss).unwrap());
            assert_eq!(b'c' as u64, number::decode_var_u64(&mut ss).unwrap());
        }
    })
}
