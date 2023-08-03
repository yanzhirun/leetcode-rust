pub fn max_area(height: Vec<i32>) -> i32 {
    //双指针收缩搜索空间
    let mut max = 0;
    let mut l = 0;
    let mut r = height.len() - 1;

    while l < r {
        if height[l] > height[r] {
            max = i32::max(max, height[r] * (r - l) as i32);
            r -= 1;
        } else {
            max = i32::max(max, height[l] * (r - l) as i32);
            l += 1;
        }
    }
    max
    // 暴力 超时
    //     let mut max = 0;
    //     for i in 0..height.len() {
    //         for j in (i + 1)..height.len() {
    //             let res = i32::min(height[i], height[j]) * (j - i) as i32;
    //             max = i32::max(max, res);
    //         }
    //     }
    //     max
}

#[test]
pub fn test_11() {
    let height1 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(49, max_area(height1));

    let height2 = vec![1, 1];
    assert_eq!(1, max_area(height2))
}
