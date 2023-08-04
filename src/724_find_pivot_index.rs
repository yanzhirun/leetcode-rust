pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut pre_sum = 0;

    for (idx, x) in nums.iter().enumerate() {
        if pre_sum == sum - x - pre_sum {
            return idx as i32;
        }
        pre_sum += x;
    }
    -1
}

#[cfg(test)]
#[test]
pub fn test_724() {
    let num = vec![1, 7, 3, 6, 5, 6];
    assert_eq!(3, pivot_index(num));

    let num = vec![2, 1, -1];
    assert_eq!(0, pivot_index(num));
}
