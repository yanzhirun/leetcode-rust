pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum_lk: i32 = nums[..k as usize].iter().sum();
    let mut max = sum_lk;

    for i in k as usize..nums.len() {
        sum_lk = sum_lk - nums[i - k as usize] + nums[i as usize];
        max = i32::max(max, sum_lk);
    }
    max as f64 / k as f64
}

#[cfg(test)]
#[test]
pub fn test_643() {
    let nums: Vec<i32> = vec![1, 12, -5, -6, 50, 3];
    let k: i32 = 4;

    assert_eq!(12.75, find_max_average(nums, k));

    let nums: Vec<i32> = vec![5];
    let k: i32 = 1;

    assert_eq!(5.0, find_max_average(nums, k));
}
