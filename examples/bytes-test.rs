#![cfg(test)]

use bytes::{Buf, Bytes};

#[test]
fn test_get_will_consume_len_of_bytes() {
    let mut bytes = Bytes::from_static(b"hello");
    assert_eq!(5, bytes.len());

    let first_char = bytes.get_u8() as char;
    assert_eq!('h', first_char);

    // consume len
    assert_eq!(4, bytes.len());
}

#[test]
fn test_slice() {
    let mut bytes = Bytes::from_static(b"hello world");
    let slice = bytes.slice(0..5);
    assert_eq!(slice, &b"hello"[..]);
}

#[test]
fn test_slice_ref() {
    let mut bytes = Bytes::from_static(b"hello world");
    // slice_ref将接收原始bytes中的一个子`&u8[..]`（必须来自于原始bytes），将它返回成一个新的`Bytes`。
    // 此操作O(1)
    let new_bytes = bytes.slice_ref(&bytes[0..5]);
    assert_eq!(new_bytes, &b"hello"[..]);
}


fn main() {}