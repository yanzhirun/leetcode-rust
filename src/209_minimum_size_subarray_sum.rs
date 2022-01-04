pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut res = i32::MAX;
    let mut sublen;
    let mut sum = 0;
    let mut left = 0;

    for i in 0..nums.len() {
        sum += nums[i];

        while sum >= target {
            sublen = i - left + 1;
            res = if sublen as i32 > res {
                res
            } else {
                sublen as i32
            };
            //res = sublen > res ? res : sublen;
            sum -= nums[left];
            left += 1;
        }
    }

    if res == i32::MAX {
        0
    } else {
        res
    }
}

#[cfg(test)]
#[test]
pub fn test_206() {
    let test_case_nums1: Vec<i32> = vec![2, 3, 1, 2, 4, 3];
    let test_case_target1: i32 = 7;
    let test_case_nums2: Vec<i32> = vec![1, 4, 4];
    let test_case_target2: i32 = 4;
    let test_case_nums3: Vec<i32> = vec![1, 1, 1, 1, 1, 1, 1, 1];
    let test_case_target3: i32 = 11;

    assert_eq!(2, min_sub_array_len(test_case_target1, test_case_nums1));
    assert_eq!(1, min_sub_array_len(test_case_target2, test_case_nums2));
    assert_eq!(0, min_sub_array_len(test_case_target3, test_case_nums3));
}
