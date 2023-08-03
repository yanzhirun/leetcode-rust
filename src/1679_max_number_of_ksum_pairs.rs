use std::collections::HashMap;

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut hm = HashMap::new();
    let mut count = 0;

    for x in nums {
        if *hm.get(&(k - x)).unwrap_or(&0) > 0 {
            // let s = *hm.get_mut(&(k-x)).unwrap();
            *hm.get_mut(&(k - x)).unwrap() -= 1;
            count += 1;
        } else {
            *hm.entry(x).or_insert(0) += 1;
        }
    }
    count
    // let mut count = 0;
    // let mut l = 0;
    // let mut r = nums.len() - 1;
    // let mut nums = nums.clone();

    // nums.sort();
    // while l < r {
    //     if &nums[l] + &nums[r] == k {
    //         count += 1;
    //         l += 1;
    //         r -= 1;
    //     } else if &nums[l] + &nums[r] > k {
    //         r -= 1;
    //     } else {
    //         l += 1;
    //     }
    // }
    // count
}

#[cfg(test)]
#[test]
pub fn test_1679() {
    let nums1 = vec![1, 2, 3, 4];
    let k1 = 5;

    assert_eq!(2, max_operations(nums1, k1));
    let nums2 = vec![3, 1, 3, 4, 3];
    let k2 = 6;
    assert_eq!(1, max_operations(nums2, k2));
}
