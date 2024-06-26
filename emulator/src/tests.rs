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
    let n1 = 5;
    let n2 = -9;
    
    assert_eq!(ones_complement32(n1), 5);
    assert_eq!(ones_complement32(n2), 0xFFFFFFF6)
}
