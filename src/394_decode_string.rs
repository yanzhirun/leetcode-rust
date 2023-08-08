pub fn decode_string(s: String) -> String {
    let mut stack: Vec<(usize, String)> = Vec::new();
    let mut times: Vec<char> = Vec::new();
    let mut str = String::new();

    for ch in s.chars() {
        match ch {
            '[' => {
                if let Ok(nn) = times.iter().cloned().collect::<String>().parse::<usize>() {
                    stack.push((nn, str.clone()));
                    times.clear();
                    str.clear();
                };
            },
            ']' => {
                if let Some(last) = stack.pop() {
                    str = last.1 + str.repeat(last.0).as_str();
                }
            },
            '0'..='9' => times.push(ch),
            _ => str.push(ch),
        }
    }
    str
}

#[cfg(test)]
#[test]
pub fn test_394() {
    let s: String = "3[a]2[bc]".to_string();
    assert_eq!("aaabcbc".to_string(), decode_string(s));

    let s: String = "2[abc]3[cd]ef".to_string();
    assert_eq!("abcabccdcdcdef".to_string(), decode_string(s));

}
