type BitIter<'a> = std::slice::Iter<'a, u32>;

fn parse_x_bits(iter: &mut BitIter, x: usize) -> u32 {
    let mut res = 0;
    for _ in 1..=x {
        res <<= 1;
        res |= *iter.next().unwrap();
    }
    res
}

fn parse_version(bin: &mut BitIter) -> u32 {
    print!("V");
    parse_x_bits(bin, 3)
}

fn parse_type(bin: &mut BitIter) -> u32 {
    print!("T");
    parse_x_bits(bin, 3)
}

fn parse_literal(bin: &mut BitIter) -> (u64, usize) {
    let mut groups_read = 0;
    let mut result = 0u64;
    let mut bits_read = 0;
    loop {
        print!("L");
        let more_groups = bin.next().unwrap();
        bits_read += 1;
        let hex = parse_x_bits(bin, 4);
        bits_read += 4;
        result <<= 4;
        result |= hex as u64;
        groups_read += 1;
        if *more_groups == 0 {
            break;
        }
    }
    assert!(groups_read <= 16);
    (result, bits_read)
}

fn parse_size_based_packets(bin: &mut BitIter) -> (u32, usize) {
    let mut result = 0;
    let size = parse_x_bits(bin, 15) as usize;
    print!("S{}", size);
    let mut read = 15usize;
    while read < size {
        let (value, pack_read) = parse_packet(bin);
        result += value;
        read += pack_read;
    }
    //assert!(read == size);
    (result, read)
}

fn parse_count_based_packets(bin: &mut BitIter) -> (u32, usize) {
    let mut result = 0;
    let mut read = 0;
    let count = parse_x_bits(bin, 11);
    print!("C{}", count);
    read += 1;
    for _ in 1..=count {
        let (value, pack_read) = parse_packet(bin);
        result += value;
        read += pack_read;
    }
    (result, read)
}

fn parse_operator(bin: &mut BitIter) -> (u32, usize) {
    let length_type = bin.next().unwrap();
    if *length_type == 0 {
        let (sz_sum, sz_read) = parse_size_based_packets(bin);
        return (sz_sum, sz_read + 1);
    } else {
        let (cnt_sum, cnt_read) = parse_count_based_packets(bin);
        return (cnt_sum, cnt_read + 1);
    }
}

fn parse_packet(bin: &mut BitIter) -> (u32, usize) {
    print!("(");
    let mut sum = 0;
    let mut read = 0usize;
    let version = parse_version(bin);
    let typ = parse_type(bin);
    read += 6;
    if typ == 4 {
        let (_, lit_read) = parse_literal(bin);
        sum += version;
        read += lit_read
    } else {
        let (op_sum, op_read) = parse_operator(bin);
        sum += op_sum;
        read += op_read;
    }
    print!(")");
    (sum, read)
}

fn hex_to_bin(hex: &str) -> Vec<u32> {
    let line = hex.lines().next().unwrap();
    let bin_vec = line
        .chars()
        .map(|x| match x {
            '0'..='9' | 'A'..='F' => x.to_digit(16).unwrap(),
            _ => panic!("Unknown char {}", x),
        })
        .map(|n| [(n & 0x8) >> 3, (n & 0x4) >> 2, (n & 0x2) >> 1, n & 0x1])
        .flatten()
        .collect::<Vec<_>>();
    bin_vec
}

fn main() {
    let hex = include_str!("day16.txt");
    let bin = hex_to_bin(hex);
    let (s, _) = parse_packet(&mut bin.iter());
    println!("");
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bin_iter() {
        let x = "ABC0123";
        let bin = hex_to_bin(x);
        let res: Vec<_> = [
            1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1,
        ]
        .to_vec();
        assert_eq!(bin, res);
    }

    #[test]
    fn test_parse_version() {
        let hex = "8";
        let bin = hex_to_bin(hex);
        let mut it = bin.iter();
        let result = parse_version(&mut it);
        assert_eq!(result, 4);
        assert_eq!(it.next(), Some(&0));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_parse_type() {
        let hex = "8";
        let bin = hex_to_bin(hex);
        let mut it = bin.iter();
        let result = parse_type(&mut it);
        assert_eq!(result, 4);
        assert_eq!(it.next(), Some(&0));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_parse_literal() {
        let hex = "D2FE28";
        let bin = hex_to_bin(hex);
        let res: Vec<_> = [
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ]
        .to_vec();
        assert_eq!(bin, res);
        let mut it = bin.iter();
        let version = parse_version(&mut it);
        let typ = parse_type(&mut it);
        let (result, read) = parse_literal(&mut it);
        assert_eq!(version, 6);
        assert_eq!(typ, 4);
        assert_eq!(result, 2021);
        assert_eq!(read, 15);
    }

    #[test]
    fn test_parse_packet_only_literal() {
        let hex = "D2FE28";
        let bin = hex_to_bin(hex);
        let mut it = bin.iter();
        let (result, read) = parse_packet(&mut it);
        assert_eq!(result, 6);
        assert_eq!(read, 21);
    }

    #[test]
    fn test_1() {
        let hex = "8A004A801A8002F478";
        let bin = hex_to_bin(hex);
        let mut it = bin.iter();
        let (result, _) = parse_packet(&mut it);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_2() {
        let hex = "620080001611562C8802118E34";
        let bin = hex_to_bin(hex);
        let mut it = bin.iter();
        let (result, _) = parse_packet(&mut it);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_3() {
        let hex = "C0015000016115A2E0802F182340";
        let bin = hex_to_bin(hex);
        let mut it = bin.iter();
        let (result, _) = parse_packet(&mut it);
        assert_eq!(result, 23);
    }

    #[test]
    fn test_4() {
        let hex = "A0016C880162017C3686B18A3D4780";
        let bin = hex_to_bin(hex);
        let mut it = bin.iter();
        let (result, _) = parse_packet(&mut it);
        assert_eq!(result, 31);
    }
}
