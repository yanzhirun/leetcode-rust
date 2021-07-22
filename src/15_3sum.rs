pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut nums_sort: Vec<i32> = nums.clone();
    if nums.len() < 3 {
        return res;
    }
    nums_sort.sort();

    if nums_sort[0] > 0 {
        return res;
    }

    for i in 0..(nums_sort.len() - 2) {
        let target = -nums_sort[i];

        if i > 0 && nums_sort[i] == nums_sort[i - 1] {
            continue;
        }

        let mut l = i + 1;
        let mut r = nums_sort.len() - 1;

        while l < r {
            let tempsum = nums_sort[l] + nums_sort[r];
            match tempsum.cmp(&target) {
                std::cmp::Ordering::Equal => {
                    res.push(vec![nums_sort[i], nums_sort[l], nums_sort[r]]);
                    while l < r && nums_sort[l] == nums_sort[l + 1] {
                        l += 1;
                    }
                    while l < r && nums_sort[r] == nums_sort[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                }
                std::cmp::Ordering::Greater => {
                    r -= 1;
                }
                std::cmp::Ordering::Less => l += 1,
            }
        }
    }

    res
}

#[test]
pub fn test_three_sum() {
    let a = vec![-1, 0, 1, 1, 1, -11, 1, 2, 1, 1];

    let res = three_sum(a);
    println!("res is {:?}", res);

    // assert_eq!(&[vec![-1,4,0]], &res[..]);

    assert_eq!(&[vec![-1, 0, 1]], &res[..]);
}

