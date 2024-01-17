use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        //hashset
        // let hsnums1: HashSet<i32> = HashSet::from_iter(nums1);
        // let hsnums2: HashSet<i32> = HashSet::from_iter(nums2);
        let hsnums1: HashSet<&i32> = nums1.iter().collect();
        let hsnums2: HashSet<&i32> = nums2.iter().collect();

        // vec![
        //     hsnums1.difference(&hsnums2).copied().collect(),
        //     hsnums2.difference(&hsnums1).copied().collect(),
        // ]

        vec![
            hsnums1.difference(&hsnums2).map(|x| **x).collect(),
            hsnums2.difference(&hsnums1).map(|x| **x).collect(),
        ]

        //暴力解法，耗时长
        // let mut res: Vec<Vec<i32>> = Vec::new();
        // let mut res0: Vec<i32> = Vec::new();
        // let mut res1: Vec<i32> = Vec::new();

        // for x in nums1.iter() {
        //     if !nums2.contains(x) && !res0.contains(x) {
        //         res0.push(*x);
        //     }
        // }

        // for x in nums2.iter() {
        //     if !nums1.contains(x) && !res1.contains(x) {
        //         res1.push(*x);
        //     }
        // }

        // res.push(res0);
        // res.push(res1);
        // res
    }

#[cfg(test)]
#[test]
pub fn test_2215() {
    let nums1 = vec![1,2,3];
    let nums2 = vec![2,4,6];

    assert_eq!(vec![vec![3,1], vec![6,4]], find_difference(nums1, nums2));
}
