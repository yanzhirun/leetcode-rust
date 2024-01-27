pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(first: usize, nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if first == nums.len() {
            println!("{:?}", nums);
            ans.push(nums.to_vec());
            return;
        }
        for i in first..nums.len() {
            nums.swap(first, i);
            dfs(first + 1, nums, ans);
            nums.swap(first, i);
        }
    }
    let mut ans = Vec::new();
    let mut nums = nums;
    dfs(0, &mut nums, &mut ans);
    ans
    //// Need to add itertools = "0.12.0" into Cargo.toml, online can not support it
    // use itertools::Itertools;

    // let nums = nums;
    // let len = nums.len();
    // let perm = nums.into_iter().permutations(len);

    // let res = perm.collect_vec();
    // println!("get perm {:?}", res);
    // res
}

#[cfg(test)]
#[test]
pub fn test_48() {
    use std::collections::HashSet;

    let input = vec![1, 2, 3];
    let res = vec![
        [1, 2, 3],
        [1, 3, 2],
        [2, 1, 3],
        [2, 3, 1],
        [3, 1, 2],
        [3, 2, 1],
    ];
    let ress: Vec<Vec<i32>> = res.iter().map(|&x| x.to_vec()).collect();
    let hashmapres: HashSet<_> = ress.iter().collect();
    let ans: Vec<Vec<i32>> = permute(input);
    let hashmapans: HashSet<_> = ans.iter().collect();

    assert_eq!(hashmapres, hashmapans);
}
