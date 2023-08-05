pub fn move_zeroes(nums: &mut Vec<i32>) {
    let len = nums.len();
    nums.retain(|&x| x != 0);
    nums.resize(len, 0);
//     let add = len - nums.len();
//     let mut append_zero = vec![0i32; add];
//     nums.append(&mut append_zero);
}

#[cfg(test)]
#[test]
pub fn test_283() {
    let mut arr = vec![0, 1, 0, 3, 12];
    let mut arr1 = vec![0];
    move_zeroes(&mut arr);
    move_zeroes(&mut arr1);

    assert_eq!(vec![1, 3, 12, 0, 0], arr);
    assert_eq!(vec![0], arr1);
}
