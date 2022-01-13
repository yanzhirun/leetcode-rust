pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut res = vec![vec![0; n as usize];n as usize];
    let mut start: usize = 0;
    let mut loopn = n / 2;
    let mid = loopn as usize;
    let mut offset = 1;
    let mut count = 1;

    while loopn > 0 {
        let dist = (n as usize) - offset;

        for top in start..=dist {
            res[start][top] = count;
            count += 1;
        }

        for right in (start + 1)..=dist {
            res[right][dist] = count;
            count += 1;
        }

        for bottle in (start..dist).rev() {
            res[dist][bottle] = count;
            count += 1;
        }

        for left in ((start + 1)..dist).rev() {
            res[left][start] = count;
            count +=1;
        }

        start += 1;
        offset += 1;
        loopn -= 1;
    }

    if n % 2 == 1 {
        res[mid][mid] = count;
    }

    res
}


#[cfg(test)]
#[test]
pub fn test_59() {
    let test_case_num1: i32 = 1;
    let test_case_res1: Vec<Vec<i32>>  = vec![vec![1]];
    let test_case_num2: i32 = 3;
    let test_case_res2: Vec<Vec<i32>>  = vec![vec![1,2,3], vec![8, 9, 4], vec![7, 6, 5]];

    assert_eq!(test_case_res1, generate_matrix(test_case_num1));
    assert_eq!(test_case_res2, generate_matrix(test_case_num2));
}
