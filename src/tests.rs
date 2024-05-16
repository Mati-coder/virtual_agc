use crate::instructions::*;

#[test]
fn test_add_positive() { 
    // Setup
    let a = 0b0000000000000011; // 3
    let b = 0b0111111111111101; // -2

    let sum = add_modified(a, b);

    assert_eq!(sum, 0b0000000000000001); // 1
}

#[test]
fn test_add_negative() { 
    // Setup
    let a = 0b0000000000000011; // 3
    let b = 0b0111111111111010; // -5

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
    // Setup
    let a = 0b0100000000000000; // -16383
    let b = 0b0111111111111110; // -1

    let sum = add_modified(a, b);

    assert_eq!(sum, 0b1011111111111111); // 16383 / -0 with overfloww
}