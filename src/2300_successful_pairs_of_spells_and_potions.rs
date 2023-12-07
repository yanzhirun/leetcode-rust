pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut res = Vec::new();
    let len = potions.len();
    let mut pot = potions;
    pot.sort();

    for &x in spells.iter() {
        let idx = pot
            .partition_point(|&val| (val as i64 * x as i64) < success);
        res.push((len - idx) as i32);
    }
    res
}

#[cfg(test)]
#[test]
pub fn test_2300() {
    let input1:(Vec<i32>, Vec<i32>, i64) = (vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7);
    let (x, y, z) = input1;
    assert_eq!(successful_pairs(x, y, z), vec![4, 0, 3]);
}

