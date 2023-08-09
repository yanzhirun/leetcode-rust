use std::collections::VecDeque;

pub fn predict_party_victory(senate: String) -> String {
    let mut radiant = VecDeque::new();
    let mut dire = VecDeque::new();

    senate.chars().enumerate().for_each(|(i, c)| {
        if c == 'R' {
            radiant.push_back(i);
        } else {
            dire.push_back(i);
        }
    });

    while !radiant.is_empty() && !dire.is_empty() {
        let (r, d) = (radiant.pop_front().unwrap(), dire.pop_front().unwrap());
        if r < d {
            radiant.push_back(r + senate.len());
        } else {
            dire.push_back(d + senate.len());
        }
    }

    if radiant.is_empty() {
        "Dire".to_string()
    } else {
        "Radiant".to_string()
    }
}

#[cfg(test)]
#[test]
pub fn test_724() {
    let senate = "RD".to_string();
    assert_eq!("Radiant".to_string(), predict_party_victory(senate));

    let senate = "RDD".to_string();
    assert_eq!("Dire".to_string(), predict_party_victory(senate));
}
