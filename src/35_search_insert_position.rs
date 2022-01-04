
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right): (i32, i32) = (0, nums.len() as i32);

    while left < right {
        let middle: i32 = left + (right - left) / 2;

        if target > nums[middle as usize] {
            left = middle + 1;
        } else if target < nums[middle as usize] {
            right = middle;
        } else {
            return middle;
        }
    }

    return right;
}

#[cfg(test)]
#[test]
pub fn test_35() {
    let test_case_nums1: Vec<i32> = vec![1, 3, 5, 6];
    let test_case_target1: i32 = 5;
    let test_case_nums2: Vec<i32> = vec![1, 3, 5, 6];
    let test_case_target2: i32 = 2;
    let test_case_nums3: Vec<i32> = vec![1, 3, 5, 6];
    let test_case_target3: i32 = 0;
    let test_case_nums4: Vec<i32> = vec![1];
    let test_case_target4: i32 = 0;

    assert_eq!(2, search_insert(test_case_nums1, test_case_target1));
    assert_eq!(1, search_insert(test_case_nums2, test_case_target2));
    assert_eq!(0, search_insert(test_case_nums3, test_case_target3));
    assert_eq!(0, search_insert(test_case_nums4, test_case_target4));
}
