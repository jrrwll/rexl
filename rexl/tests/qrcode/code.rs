use rexl::qrcode::QrCode;

fn parse_bin(v: Vec<char>) -> u8 {
    let bin: String = v.into_iter().collect();
    println!("parse binary number from {}", &bin);
    match u8::from_str_radix(&bin, 2) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {}", e);
            0u8
        }
    }
}

#[test]
fn encode_nums() {
    // 0001 0000001000 0000001100 0101011001 1000011
    // 0001 0000|001000 00|00001100 | 01010110|01 100001|1
    // 00010000  16
    // 00100000  32
    // 00001100  12
    // 01010110  86
    // 01100001  97
    // 1         1
    let qr = QrCode::from_version(1);
    match qr.encode_nums("01234567") {
        Ok(v) => {
            let s: String = "0001 0000001000 0000001100 0101011001 1000011"
                .chars()
                .filter(|c| *c != ' ')
                .collect();
            let mut u = vec![];
            let mut i = vec![];
            for b in s.chars() {
                if i.len() != 8 {
                    i.push(b);
                    continue
                }
                u.push(parse_bin(i));
                i = vec![b];
            }
            u.push(parse_bin(i));

            println!("result: {:?}, expect: {:?}", &v, &u);
            assert_eq!(&v, &u);
        }
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }
}
