use std::collections::HashMap;

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let mut hm: HashMap<&Vec<i32>, i32> = HashMap::new();

    grid.iter().for_each(|x| *hm.entry(x).or_insert(0) += 1);
    println!("get hashmap is {:?}", hm);

    let mut res = 0;
    for i in 0..grid.len() {
        let mut colv: Vec<i32> = Vec::new();
        for j in 0..grid.len() {
            colv.push(grid[j][i]);
        }
        res += hm.get(&colv).unwrap_or(&0);
    }
    res
}

// pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
//     let mut hm: HashMap<Vec<i32>, i32> = HashMap::new();
//     for x in grid.iter().cloned() {
//         *hm.entry(x).or_insert(0) += 1;
//     }
//     println!("get hashmap is {:?}", hm);

//     let mut res = 0;
//     for i in 0..grid.len() {
//         let mut colv: Vec<i32> = Vec::new();
//         for j in 0..grid.len() {
//             colv.push(grid[j][i]);
//         }
//         res += hm.get(&colv).unwrap_or(&0);
//     }
//     res
// }

#[cfg(test)]
#[test]
pub fn test_1657() {
    let grid = vec![vec![3,2,1], vec![1,7,6], vec![2,7,7]];
    assert_eq!(1, equal_pairs(grid));
    let grid = vec![vec![3,1,2,2], vec![1,4,4,5], vec![2,4,2,2], vec![2,4,2,2]];
    assert_eq!(3, equal_pairs(grid));
}

