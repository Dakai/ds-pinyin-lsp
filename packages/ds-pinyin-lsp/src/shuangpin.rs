pub fn shuangpin_to_quanpin(sp: &str, scheme: &str) -> String {
    if scheme == "flypy" {
        return flypy_to_quanpin(sp);
    }
    sp.to_string()
}

fn flypy_to_quanpin(sp: &str) -> String {
    let mut quanpin = String::new();
    let chars: Vec<char> = sp.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i + 1 < chars.len() {
            let initial_char = chars[i];
            let final_char = chars[i + 1];
            quanpin.push_str(&combine_flypy(initial_char, final_char));
            i += 2;
        } else {
            // Dangling initial character
            let initial_char = chars[i];
            quanpin.push_str(flypy_initial(initial_char));
            i += 1;
        }
    }
    quanpin
}

fn flypy_initial(c: char) -> &'static str {
    match c {
        'q' => "q",
        'w' => "w",
        'r' => "r",
        't' => "t",
        'y' => "y",
        'u' => "sh",
        'i' => "ch",
        'p' => "p",
        's' => "s",
        'd' => "d",
        'f' => "f",
        'g' => "g",
        'h' => "h",
        'j' => "j",
        'k' => "k",
        'l' => "l",
        'z' => "z",
        'x' => "x",
        'c' => "c",
        'v' => "zh",
        'b' => "b",
        'n' => "n",
        'm' => "m",
        'a' => "a",
        'o' => "o",
        'e' => "e",
        _ => "",
    }
}

fn combine_flypy(initial_char: char, final_char: char) -> String {
    if "aoe".contains(initial_char) {
        return match (initial_char, final_char) {
            ('a', 'a') => "a",
            ('o', 'o') => "o",
            ('e', 'e') => "e",
            ('a', 'i') => "ai",
            ('a', 'n') => "an",
            ('a', 'o') => "ao",
            ('o', 'u') => "ou",
            ('e', 'n') => "en",
            ('e', 'r') => "er",
            ('a', 'h') => "ang",
            ('e', 'g') => "eng",
            _ => "",
        }
        .to_string();
    }

    let initial = flypy_initial(initial_char);
    if initial.is_empty() {
        return String::new();
    }

    let final_str = match final_char {
        'a' => "a",
        'b' => "in",
        'c' => "ao",
        'd' => "ai",
        'e' => "e",
        'f' => "en",
        'g' => "eng",
        'h' => "ang",
        'i' => "i",
        'j' => "an",
        'k' => {
            if matches!(initial_char, 'g' | 'k' | 'h' | 'v' | 'i' | 'u') {
                "uai"
            } else {
                "ing"
            }
        }
        'l' => {
            if matches!(initial_char, 'j' | 'q' | 'x' | 'y' | 'n' | 'l') {
                "iang"
            } else {
                "uang"
            }
        }
        'm' => "ian",
        'n' => "iao",
        'o' => {
            if matches!(initial_char, 'b' | 'p' | 'm' | 'f' | 'y') {
                "o"
            } else {
                "uo"
            }
        }
        'p' => "ie",
        'q' => "iu",
        'r' => "uan",
        's' => {
            if matches!(initial_char, 'j' | 'q' | 'x') {
                "iong"
            } else {
                "ong"
            }
        }
        't' => {
            if matches!(initial_char, 'n' | 'l') {
                "ve"
            } else {
                "ue"
            }
        }
        'u' => "u",
        'v' => {
            if matches!(initial_char, 'n' | 'l') {
                "v"
            } else {
                "ui"
            }
        }
        'w' => "ei",
        'x' => {
            if matches!(initial_char, 'g' | 'k' | 'h' | 'v' | 'i' | 'u' | 'r') {
                "ua"
            } else {
                "ia"
            }
        }
        'y' => "un",
        'z' => "ou",
        _ => return String::new(),
    };

    format!("{}{}", initial, final_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flypy_to_quanpin() {
        assert_eq!(flypy_to_quanpin("nihc"), "nihao");
        assert_eq!(flypy_to_quanpin("yh"), "yang");
        assert_eq!(flypy_to_quanpin("aa"), "a");
        assert_eq!(flypy_to_quanpin("ai"), "ai");
        assert_eq!(flypy_to_quanpin("ah"), "ang");
        assert_eq!(flypy_to_quanpin("vi"), "zhi");
        assert_eq!(flypy_to_quanpin("ui"), "shi");
        assert_eq!(flypy_to_quanpin("ii"), "chi");
        assert_eq!(flypy_to_quanpin("lt"), "lve");
        assert_eq!(flypy_to_quanpin("jt"), "jue");
    }

    #[test]
    fn test_flypy_dangling() {
        assert_eq!(flypy_to_quanpin("nih"), "nih");
        assert_eq!(flypy_to_quanpin("u"), "sh");
        assert_eq!(flypy_to_quanpin("v"), "zh");
        assert_eq!(flypy_to_quanpin("i"), "ch");
    }
}
