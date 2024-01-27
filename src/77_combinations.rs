pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    pub fn dfs(n: usize, k: usize, ans: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if ans.len() == k {
            res.push(ans.to_vec());
            return;
        }
        let start = if ans.len() == 0 {
            1
        } else {
            ans[ans.len() - 1] + 1
        };
        for cur in start as usize..=n {
            ans.push(cur as i32);
            dfs(n, k, ans, res);
            ans.pop();
        }
    }

    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut ans: Vec<i32> = Vec::new();
    dfs(n as usize, k as usize, &mut ans, &mut res);
    res
}

#[cfg(test)]
#[test]
pub fn test_77() {
    let (n1, k1) = (4, 2);
    let ans = [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]];
    let ans: Vec<Vec<i32>> = ans.to_vec().iter().map(|&v| v.to_vec()).collect();

    let res = combine(n1, k1);
    assert_eq!(ans, res);
}
