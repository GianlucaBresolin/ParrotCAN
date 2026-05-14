const fn parse_ids(s: &str) -> ([u16; 32], usize) {
    if s.as_bytes().is_empty() {
        return ([0u16; 32], 0);
    }
    
    let bytes = s.as_bytes();
    let mut ids = [0u16; 32];
    let mut count = 0usize;
    let mut acc = 0u16;
    let mut i = 0usize;

    while i < bytes.len() {
        let b = bytes[i];
        match b {
            b'0'..=b'9' => acc = acc * 16 + (b - b'0') as u16,
            b'a'..=b'f' => acc = acc * 16 + (b - b'a' + 10) as u16,
            b'A'..=b'F' => acc = acc * 16 + (b - b'A' + 10) as u16,
            b'x' | b'X' => {}
            b',' => {
                if count < 32 { ids[count] = acc & 0x7FF; count += 1; }
                acc = 0;
            }
            _ => {}
        }
        i += 1;
    }
    
    if count < 32 { ids[count] = acc & 0x7FF; count += 1; }

    (ids, count)
}

static MY_IDS_RAW:         ([u16; 32], usize) = parse_ids(env!("MY_IDS"));
static INTERESTED_IDS_RAW: ([u16; 32], usize) = parse_ids(env!("INTERESTED_IDS"));

pub fn my_ids()         -> &'static [u16] { &MY_IDS_RAW.0[..MY_IDS_RAW.1] }
pub fn interested_ids() -> &'static [u16] { &INTERESTED_IDS_RAW.0[..INTERESTED_IDS_RAW.1] }
