pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for (idx, val) in nums.iter().enumerate() {
        let need = target - *val;
        match hashmap.get(&need) {
            Some(&needidx) => return vec![idx as i32, needidx],
            None => hashmap.insert(*val, idx as i32),
        };
    }
    vec![]
}

///run cargo test -- --show-output
#[cfg(test)]
#[test]
fn test_1() {
    let a = vec![1, 2, 3, 4, 9];

    let res = two_sum(a, 10);
    println!("res is {:?}", res);

    assert_eq!(&[4, 0], &res[..]);
}
