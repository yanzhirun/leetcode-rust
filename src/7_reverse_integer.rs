
pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut ans = 0;

    while x != 0 {
        if ans > i32::MAX / 10 || ans < i32::MIN / 10 {
            return 0;
        }

        ans = ans * 10 + x % 10;
        x = x / 10;
    }
    ans
}

#[cfg(test)]
#[test]
pub fn test_7() {
    let test_case1 = 0;
    let test_case2 = -123456;
    let test_case3 = 990123;
    let test_case4 = 1234567899;

    println!("i32 max: {} ; min:{}", i32::MAX, i32::MIN);
    assert_eq!(0, reverse(test_case1));
    assert_eq!(-654321, reverse(test_case2));
    assert_eq!(321099, reverse(test_case3));
    assert_eq!(0, reverse(test_case4));
}
