use super::*;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_large_numbers() {
    let result = add(u64::MAX - 1, 1);
    assert_eq!(result, u64::MAX);
}
