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
        result <<= 1;
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
    print!("S{}", count);
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

fn main() {
    let hex = include_str!("day16.txt");
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
    let mut bin = bin_vec.iter();
    let (s, _) = parse_packet(&mut bin);
    println!("");
    println!("{}", s);
}
