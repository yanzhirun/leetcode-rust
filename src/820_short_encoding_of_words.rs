pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
    let mut res = 0i32;
    let mut words: Vec<String> = words
        .into_iter()
        .map(|word| word.chars().rev().collect::<String>())
        .collect();
    words.sort(); // 排序默认已经是字典排序,
    println!("words is {:?}", words);
    (0..words.len()).for_each(|i| {
        if i + 1 < words.len() && words[i + 1].starts_with(&words[i]) {
            res += 0;
        } else {
            res += words[i].len() as i32 + 1;
        }
    });
    res
}

#[cfg(test)]
#[test]
pub fn test_820() {
    let input = ["time", "me", "bell"].to_vec();
    let input: Vec<String> = input.iter().map(|&s| s.to_string()).collect();
    let output = 10;
    assert_eq!(output, minimum_length_encoding(input));
}
