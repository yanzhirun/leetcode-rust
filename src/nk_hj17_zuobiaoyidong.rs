use std::str::FromStr;

pub fn movexy(input: String) -> (i32, i32) {
    let inv: Vec<&str> = input.split(";").collect();
    let (mut x, mut y) = (0, 0);
    for strin in inv {
        if !is_correct(strin) {
            continue;
        }

        match strin.get(0..1) {
            Some("A") => x -= i32::from_str(strin.get(1..).unwrap()).unwrap(),
            Some("D") => x += i32::from_str(strin.get(1..).unwrap()).unwrap(),
            Some("W") => y += i32::from_str(strin.get(1..).unwrap()).unwrap(),
            Some("S") => y -= i32::from_str(strin.get(1..).unwrap()).unwrap(),
            _ => panic!(),
        };
    }
    (x, y)
}

pub fn is_correct(input: &str) -> bool {
    if input.len() == 0 {
        return false;
    }
    let dirv = vec!["A", "D", "S", "W"];
    let a = input.get(0..1).unwrap();
    if !dirv.contains(&a) {
        return false;
    }
    let len = input.len();
    let digv = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    for idx in 1..len {
        let d = input.get(idx..idx + 1).unwrap();
        if !digv.contains(&d) {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[test]
pub fn test_nk_hj17() {
    let input = "A10;S20;W10;D30;X;A1A;B10A11;;A10;".to_string();
    let ans = (10, -10);

    let res = movexy(input);
    assert_eq!(ans, res);

    let input = "ABC;AKL;DA1;;".to_string();
    let ans = (0, 0);

    let res = movexy(input);
    assert_eq!(ans, res);

}
