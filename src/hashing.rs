pub fn fnv1a_32(data: &[u8]) -> u32 {
    let mut hash: u32 = 0x811c9dc5;
    for &byte in data {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(0x01000193);
    }
    hash
}

pub fn fnv1a_64(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub fn djb2(data: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for &byte in data {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

pub fn sdbm(data: &[u8]) -> u64 {
    let mut hash: u64 = 0;
    for &byte in data {
        hash = (byte as u64)
            .wrapping_add(hash << 6)
            .wrapping_add(hash << 16)
            .wrapping_sub(hash);
    }
    hash
}

fn build_crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut n: usize = 0;
    while n < 256 {
        let mut c: u32 = n as u32;
        let mut k: u32 = 0;
        while k < 8 {
            if c & 1 == 1 {
                c = 0xedb88320u32 ^ (c >> 1);
            } else {
                c >>= 1;
            }
            k += 1;
        }
        table[n] = c;
        n += 1;
    }
    table
}

pub fn crc32(data: &[u8]) -> u32 {
    let table = build_crc32_table();
    let mut crc: u32 = 0xffffffff;
    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xff) as usize;
        crc = table[index] ^ (crc >> 8);
    }
    crc ^ 0xffffffff
}

pub fn adler32(data: &[u8]) -> u32 {
    let modulus: u32 = 65521;
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = a.wrapping_add(byte as u32) % modulus;
        b = b.wrapping_add(a) % modulus;
    }
    (b << 16) | a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a_32_empty() {
        assert_eq!(fnv1a_32(b""), 0x811c9dc5);
    }

    #[test]
    fn fnv1a_32_single_byte() {
        assert_eq!(fnv1a_32(b"a"), 0xe40c292c);
    }

    #[test]
    fn fnv1a_32_is_deterministic() {
        assert_eq!(fnv1a_32(b"scripting-vm"), fnv1a_32(b"scripting-vm"));
    }

    #[test]
    fn fnv1a_64_empty() {
        assert_eq!(fnv1a_64(b""), 0xcbf29ce484222325);
    }

    #[test]
    fn fnv1a_64_nonempty_differs_from_empty() {
        assert_ne!(fnv1a_64(b"a"), fnv1a_64(b""));
        assert_ne!(fnv1a_64(b"a"), 0);
    }

    #[test]
    fn djb2_empty() {
        assert_eq!(djb2(b""), 5381);
    }

    #[test]
    fn djb2_single_byte() {
        assert_eq!(djb2(b"a"), 177670);
    }

    #[test]
    fn sdbm_empty() {
        assert_eq!(sdbm(b""), 0);
    }

    #[test]
    fn sdbm_single_byte() {
        assert_eq!(sdbm(b"a"), 97);
    }

    #[test]
    fn crc32_empty() {
        assert_eq!(crc32(b""), 0);
    }

    #[test]
    fn crc32_known_vector() {
        assert_eq!(crc32(b"123456789"), 0xcbf43926);
    }

    #[test]
    fn crc32_table_basics() {
        let table = build_crc32_table();
        assert_eq!(table.len(), 256);
        assert_eq!(table[0], 0);
    }

    #[test]
    fn adler32_empty() {
        assert_eq!(adler32(b""), 1);
    }

    #[test]
    fn adler32_known_vector() {
        assert_eq!(adler32(b"Wikipedia"), 0x11e60398);
    }

    #[test]
    fn all_functions_are_deterministic_across_samples() {
        let samples: [&[u8]; 5] = [b"", b"a", b"hash", b"checksum", b"The quick brown fox"];
        for sample in samples.iter() {
            assert_eq!(fnv1a_32(sample), fnv1a_32(sample));
            assert_eq!(fnv1a_64(sample), fnv1a_64(sample));
            assert_eq!(djb2(sample), djb2(sample));
            assert_eq!(sdbm(sample), sdbm(sample));
            assert_eq!(crc32(sample), crc32(sample));
            assert_eq!(adler32(sample), adler32(sample));
        }
    }

    #[test]
    fn empty_input_never_panics_and_has_stable_values() {
        let empty: &[u8] = b"";
        assert_eq!(fnv1a_32(empty), 0x811c9dc5);
        assert_eq!(fnv1a_64(empty), 0xcbf29ce484222325);
        assert_eq!(djb2(empty), 5381);
        assert_eq!(sdbm(empty), 0);
        assert_eq!(crc32(empty), 0);
        assert_eq!(adler32(empty), 1);
    }
}
