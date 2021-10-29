pub struct QrCode {
    pub version:      u8,
    pub size:         u8,
    // code width
    pub numeric:      u8,
    // 0001
    pub alphanumeric: u8,
    // 0010
    pub bit8:         u8,
    // 0100
    pub kanji:        u8, // 1000
}

impl QrCode {
    fn new(version: u8, size: u8, numeric: u8, alphanumeric: u8, bit8: u8, kanji: u8) -> Self {
        QrCode {
            version,
            size,
            numeric,
            alphanumeric,
            bit8,
            kanji,
        }
    }

    pub fn from_version(version: u8) -> Self {
        let size = ((version - 1) << 2) + 21;
        if version <= 9 {
            Self::new(version, size, 10, 9, 8, 8) // 1-9
        } else if version <= 26 {
            Self::new(version, size, 12, 11, 16, 10) // 10-26
        } else {
            Self::new(version, size, 14, 13, 16, 12) // 27-40
        }
    }

    pub fn encode_nums(&self, nums: &str) -> Result<Vec<u8>, String> {
        let mut bytes = vec![];

        let mut prev_bits: u8 = 0001;
        let mut expect = 4;

        let len = nums.len();
        let mut i = 0;
        let mut size_append = false;
        while i < len {
            let mut numeric = self.numeric;
            let mut remain = numeric - expect;
            let mut j = i + 3;
            let diff = j as i64 - len as i64;
            if diff > 0 {
                j = len;
                if diff == 1 {
                    numeric = 7;
                } else if diff == 2 {
                    numeric = 4;
                }
                if numeric > expect {
                    remain = numeric - expect;
                } else {
                    remain = 0;
                }
            }
            let num: u16;
            if size_append {
                num = nums[i..j].parse::<u16>().map_err(|e| e.to_string())?;
                i = j;
            } else {
                num = len as u16;
                size_append = true;
            }

            let left = if prev_bits != 0 {
                prev_bits << expect
            } else {
                0
            };
            let right = (num >> remain) as u8;
            bytes.push(left | right);

            while remain >= 8 {
                let next_remain = remain - 8;
                bytes.push(((num >> next_remain) & 0xff) as u8);
                remain = next_remain;
            }
            prev_bits = (num & ((1 << remain) - 1)) as u8;
            expect = 8 - remain;
            if i == len {
                bytes.push(prev_bits);
                break
            }
        }
        Ok(bytes)
    }
}
