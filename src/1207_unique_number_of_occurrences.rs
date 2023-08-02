use std::collections::HashSet;

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    //     let mut hs_arr: HashSet<i32> = HashSet::from_iter(arr.clone());
    let hs_arr: HashSet<&i32> = arr.iter().collect();
    let len = hs_arr.len();
    let mut count_a = vec![0; len];
    let mut idx = 0;
    for &&i in &hs_arr {
        for &x in &arr {
            if x == i {
                count_a[idx] += 1;
            }
        }
        idx += 1;
    }
    let mut res = HashSet::new();
    for i in count_a {
        if res.insert(i) == false {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[test]
pub fn test_1657() {
    let arr = vec![1, 2, 2, 1, 1, 3];
    let arr1 = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];

    assert_eq!(true, unique_occurrences(arr));
    assert_eq!(true, unique_occurrences(arr1));
}
