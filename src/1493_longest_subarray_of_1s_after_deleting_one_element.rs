// 滑动窗口
// 以右指针作为驱动，拖着左指针向前走。
// 右指针每次只移动一步，而左指针在内部 while 循环中每次可能移动多步。
// 右指针是主动前移，探索未知的新区域；左指针是被迫移动，负责寻找满足题意的区间

pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let (mut l, mut r) = (0, 0);
    let mut len;
    let mut count = 0;

    while r < nums.len() {
        if nums[r] == 0 {
            count += 1;
        }
        while count > 1 {
            if nums[l] == 0 {
                count -= 1;
            }
            l += 1;
        }
        len = r - l;
        max = usize::max(max, len);
        r += 1;
    }
    max as i32
}

#[cfg(test)]
#[test]
pub fn test_1493() {
    let nums1 = vec![1, 1, 0, 1];
    let nums2 = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
    let nums3 = vec![1, 1, 1];

    assert_eq!(3, longest_subarray(nums1));
    assert_eq!(5, longest_subarray(nums2));
    assert_eq!(2, longest_subarray(nums3));
}
