pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut res:Vec<char> = Vec::new();
        res.push(chars[0]);
        let mut count = 0;

        // if chars.len() == 1
        // {
        //     return 1;
        // }
        // if chars.len() == 0 {
        //     return 0;
        // }

        for c in chars.iter() {
            if *c != res[res.len() - 1] {
                if count > 1 {
                    // res.push(char::from_digit(count, 10).unwrap());
                    res.append(&mut count.to_string().chars().collect::<Vec<char>>());
                }
                res.push(*c);
                count = 1;
            } else {
                count += 1;
            }
        }
        if count > 1 {
            // res.push(char::from_digit(count, 10).unwrap());
            res.append(&mut count.to_string().chars().collect::<Vec<char>>());
            // res.append(&mut count.to_string().chars().collect::<Vec<char>>());
        }
        chars.clear();
        chars.append(&mut res);
        chars.len() as i32
    }

#[cfg(test)]
#[test]
pub fn test_443() {
    let mut chars1 :Vec<char>= vec!['a', 'a', 'b'];
    let mut chars2 :Vec<char>= vec!['a'; 10];

    assert_eq!(3, compress(&mut chars1));
    assert_eq!(3, compress(&mut chars2));
}
