use crate::instructions::*;

#[test]
fn test_add_positive() { 
    // Setup 16bit
    let a = 0b0000000000000011; // 3
    let b = 0b1111111111111101; // -2

    let sum = add_modified(a, b);

    assert_eq!(sum, 0b0000000000000001); // 1
}

#[test]
fn test_add_negative() { 
    // Setup 16bit
    let a = 0b0000000000000011; // 3
    let b = 0b1111111111111010; // -5

    let sum = add_modified(a, b);

    assert_eq!(sum, 0b1111111111111101); // -2
}

#[test]
fn test_add_positive_overflow() { 
    // Setup
    let a = 0b0000000000000011; // 3
    let b = 0b0011111111111110; // 16382

    let sum = add_modified(a, b);

    assert_eq!(sum, 0b0100000000000001); // -16382 / 1 with overflow
}

#[test]
fn test_add_negative_overflow() { 
    // Setup 16bit
    let a = 0b1100000000000000; // -16383
    let b = 0b1111111111111110; // -1

    let sum = add_modified(a, b);

    assert_eq!(sum, 0b1011111111111111); // 16383 / -0 with overflow
}

#[test]
fn test_ones_complement() {
    let n1 = 32767;
    let n2 = -32767;
    
    assert_eq!(ones_complement32(n1), 32767);
    assert_eq!(ones_complement32(n2), 0xFFFF8000);
    assert_eq!(ones_complement16(n1 as i16), 32767);
    assert_eq!(ones_complement16(n2 as i16), 0x8000);
}

#[test]
fn test_i32_conversion() {
    let n1 = 0;
    let n2 = 0xFFFF; // NEG ZERO
    let n3 = 8;
    let n4 = 0xFFF7;

    let n5 = 0x8001;
    let n6 = 1;

    let n7 = 0xFFFE;
    let n8 = 0x7FFE;

    assert_eq!(as_i32(n1), 0);
    assert_eq!(as_i32(n2), 0);
    assert_eq!(as_i32(n3), 8);
    assert_eq!(as_i32(n4), -8);
    assert_eq!(as_i32(n5), as_i32(n6));
    assert_eq!(as_i32(n7), as_i32(n8));
}
