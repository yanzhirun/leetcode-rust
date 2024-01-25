use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for x in nums {
        *map.entry(x).or_insert(0) += 1;
    }

    let mut mapv: Vec<(&i32, &i32)> = map.iter().collect();

    mapv.sort_by(|x, y| y.1.cmp(x.1)); //sort (key, value) by value from big to little

    let res = mapv.drain(0..k as usize).map(|(&n, _)| n).collect();

    res
}

#[cfg(test)]
#[test]
pub fn test_347() {
    let nums = vec![1, 1, 11, 22, 55, 23, 22, 1, 25, 11];
    let k = 3;
    let mut res = top_k_frequent(nums, k);
    res.sort();
    assert_eq!(res, vec![1, 11, 22]);

    let nums = vec![1, 1, 1, 1, 1, 2, 2, 3];
    let k = 2;
    assert_eq!(top_k_frequent(nums, k), vec![1, 2]);
}
