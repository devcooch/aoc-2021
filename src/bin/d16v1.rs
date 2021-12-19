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
    parse_x_bits(bin, 3)
}

fn parse_type(bin: &mut BitIter) -> u32 {
    parse_x_bits(bin, 3)
}

fn parse_literal(bin: &mut BitIter) -> u64 {
    let mut groups_read = 0;
    let mut result = 0u64;
    loop {
        let more_groups = bin.next().unwrap();
        let hex = parse_x_bits(bin, 4);
        result <<= 1;
        result |= hex as u64;
        groups_read += 1;
        if *more_groups == 0 {
            break;
        }
    }
    assert!(groups_read <= 16);
    result
}

fn parse_size_based_packets(bin: &mut BitIter) -> u32 {
    let mut result = 0;
    let size = parse_x_bits(bin, 15);
    result
}

fn parse_count_based_packets(bin: &mut BitIter) -> u32 {
    let mut result = 0;
    let count = parse_x_bits(bin, 11);
    for _ in 1..=count {
        result += parse_packet(bin);
    }
    result
}

fn parse_operator(bin: &mut BitIter) -> u32 {
    let length_type = bin.next().unwrap();
    if *length_type == 0 {
        parse_size_based_packets(bin)
    } else {
        parse_count_based_packets(bin)
    }
}

fn parse_packet(bin: &mut BitIter) -> u32 {
    let mut sum = 0;
    let version = parse_version(bin);
    let typ = parse_type(bin);
    if typ == 4 {
        parse_literal(bin);
        sum += version;
    } else {
        sum += parse_operator(bin)
    }
    sum
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
    let s = parse_packet(&mut bin);
    println!("{}", s);
}
