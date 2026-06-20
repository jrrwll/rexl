pub fn next_number(chars: &Vec<u8>, offset: usize) -> Option<(usize, bool)> {
    let mut dot = false;
    let mut permit_dot = true;
    let mut has_e = false;
    let mut has_e_sign = false;
    let mut has_pow = false;
    let mut expect_pow = false;
    let mut expect_e_sign_or_pow = false;
    let mut permit_sign = true;

    let len = chars.len();
    let mut i = offset;
    while i < len {
        let c = chars[i] as char;
        if c == '-' || c == '+' {
            if !permit_sign {
                break;
            }
            permit_sign = false;
            if has_e {
                has_e_sign = true;
                expect_pow = true;
                expect_e_sign_or_pow = false;
            }
            i += 1;
            continue;
        }
        if c.is_ascii_digit() {
            if has_e {
                has_pow = true;
                expect_pow = false;
                expect_e_sign_or_pow = false;
            }
            permit_sign = false;
        } else if c == '.' {
            if !permit_dot {
                if has_pow {
                    break;
                }
                // go back
                if has_e_sign {
                    i -= 1;
                }
                if expect_pow {
                    i -= 1;
                }
                expect_pow = false;
                break;
            }
            dot = true;
            permit_dot = false;
        } else if c == 'e' || c == 'E' {
            if has_e {
                break;
            }
            has_e = true;
            permit_sign = true;
            permit_dot = false;
            expect_e_sign_or_pow = true;
        } else {
            break;
        }

        i += 1;
    }
    if i == offset {
        return None;
    }
    // go back
    if expect_e_sign_or_pow {
        i -= 1;
    } else if expect_pow {
        i -= 2;
    }
    let floating = dot || has_e; // float case
    Some((i, floating))
}
