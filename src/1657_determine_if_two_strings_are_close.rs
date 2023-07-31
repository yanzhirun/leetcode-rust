use std::collections::HashSet;

pub fn close_strings(word1: String, word2: String) -> bool {
    let hs1: HashSet<char> = word1.chars().collect();
    let hs2: HashSet<char> = word2.chars().collect();

    if word1.len() != word2.len() {
        return false;
    }
    if !hs1
        .symmetric_difference(&hs2)
        .copied()
        .collect::<Vec<char>>()
        .is_empty()
    {
        return false;
    }
    let mut count1 = vec![0; hs1.len()];
    let mut count2 = vec![0; hs2.len()];

    let mut idx = 0;
    for hs in &hs1 {
        for x in word1.chars() {
            if hs == &x {
                count1[idx] += 1;
            }
        }
        idx += 1;
    }

    let mut idx = 0;
    for hs in &hs2 {
        for x in word2.chars() {
            if hs == &x {
                count2[idx] += 1;
            }
        }
        idx += 1;
    }
    count1.sort();
    count2.sort();
    if count1 == count2 {
        return true;
    }
    false
}

#[cfg(test)]
#[test]
pub fn test_1657() {
    let words1 = "a".to_string();
    let words2 = "aa".to_string();

    assert_eq!(false, close_strings(words1, words2));
}
