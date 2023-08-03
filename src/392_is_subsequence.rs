pub fn is_subsequence(s: String, t: String) -> bool {
    if s.len() > t.len() {
        return false;
    }

    let sc: Vec<char> = s.chars().collect();
    let mut idx = 0;

    for x in t.chars() {
        if idx == s.len() {
            return true;
        }
        if &x == &sc[idx] {
            idx +=1;
        }
    }
    if idx < s.len() {
        return false;
    } else {
        return true;
    }

//     let sc: Vec<char> = s.chars().collect();
//     let tc: Vec<char> = t.chars().collect();

//     let mut idx = 0;
//     for x in &sc {
//         let mut found = false;
//         for i in idx..t.len() {
//             if &tc[i] == x {
//                 idx = i + 1;
//                 found = true;
//                 break;
//             }
//         }
//         if !found {
//             println!("not find {}", x);
//             return false;
//         }
//     }
//     true
}

#[cfg(test)]
#[test]
pub fn test_392() {
    let s: String = "abc".to_string();
    let t: String = "ahbgdc".to_string();

    assert_eq!(true, is_subsequence(s, t));

    let s: String = "axc".to_string();
    let t: String = "ahbgdc".to_string();

    assert_eq!(false, is_subsequence(s, t));
}
