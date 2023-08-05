pub fn remove_stars(s: String) -> String {
    // Vec 模拟 栈
    let mut st = vec![];

    for c in s.chars() {
        match c {
            '*' => {
                st.pop();
            }
            _ => {
                st.push(c);
            }
        }
    }

    st.into_iter().collect()

    //     let vec_s = s.chars().collect::<Vec<char>>();
    //     let mut res_v: Vec<char> = Vec::new();
    //     let mut count = 0;

    //     for idx in (0..vec_s.len()).rev() {
    //         if vec_s[idx] == '*' {
    //             count += 1;
    //             continue;
    //         }

    //         if count > 0 {
    //             count -= 1;
    //             continue;
    //         } else {
    //             res_v.push(vec_s[idx]);
    //         }
    //     }

    //     res_v.iter().rev().collect::<String>()
}

#[cfg(test)]
#[test]
pub fn test_2390() {
    let s: String = "leet**cod*e".to_string();
    assert_eq!("lecoe".to_string(), remove_stars(s));

    let s: String = "erase*****".to_string();
    assert_eq!("".to_string(), remove_stars(s));
}
