pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut max = 0;
    let (mut l, mut r) = (0, 0);
    let mut count = 0;
    let len = nums.len();
    while r < len {
        if nums[r] == 0 {
            count += 1;
        }

        while count > k {
            if nums[l] == 0 {
                count -= 1;
            }
            l += 1;
        }
        max = i32::max(max, (r - l + 1) as i32);
        r += 1;
    }
    max
}

#[cfg(test)]
#[test]
pub fn test_1657() {
    let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    let k = 2;
    assert_eq!(6, longest_ones(nums, k));

    let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    let k = 3;
    assert_eq!(10, longest_ones(nums, k));
}
