#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Element {
    value: u32,
    depth: u32,
}

fn parse_line(l: &str) -> Vec<Element> {
    let mut d = 0;
    let mut res = Vec::new();
    for c in l.chars() {
        match c {
            '[' => d += 1,
            ']' => d -= 1,
            '0'..='9' => res.push(Element {
                value: c.to_digit(10).unwrap(),
                depth: d,
            }),
            ',' => {}
            _ => panic!("Unexpected char {}", c),
        }
    }
    res
}

fn try_explode(a: &mut Vec<Element>) -> bool {
    let explodable = a.iter().position(|x| x.depth > 4);
    if let Some(index) = explodable {
        assert!(a[index + 1].depth > 4);
        if index > 0 {
            a[index - 1].value += a[index].value;
        }
        a[index].value = 0;
        a[index].depth -= 1;
        if index + 2 < a.len() {
            a[index + 2].value += a[index + 1].value;
        }
        a.remove(index + 1);
    }
    explodable.is_some()
}

fn try_split(a: &mut Vec<Element>) -> bool {
    let splittable = a.iter().position(|x| x.value >= 10);
    if let Some(index) = splittable {
        a[index].depth += 1;
        a.insert(
            index + 1,
            Element {
                value: (a[index].value as f32 / 2.0).ceil() as u32,
                depth: a[index].depth,
            },
        );
        a[index].value /= 2;
    }
    splittable.is_some()
}

fn sum_a_b<'a>(a: &'a mut Vec<Element>, b: &Vec<Element>) -> &'a mut Vec<Element> {
    a.extend_from_slice(b);
    for x in a.iter_mut() {
        x.depth += 1;
    }
    while try_explode(a) || try_split(a) {}
    a
}

fn magnitude(a: &[Element]) -> u32 {
    let mut curr: Vec<_> = a.to_vec();
    for d in (1u32..=4).rev() {
        let mut new = Vec::new();
        let mut it = curr.iter();
        while let Some(x) = it.next() {
            if x.depth == d {
                let y = it.next().unwrap();
                new.push(Element {
                    value: 3 * x.value + 2 * y.value,
                    depth: d - 1,
                });
            } else {
                new.push(*x);
            }
        }
        curr = new;
    }
    curr[0].value
}

fn main() {
    let data = include_str!("day18.txt");
    let mut nums = data.lines().map(|line| parse_line(line));
    let mut result = nums.next().unwrap().to_vec();
    nums.fold(&mut result, |acc, x| sum_a_b(acc, &x));
    println!("{}", magnitude(&result));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let x = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let res: Vec<Element> = [
            (1u32, 4u32),
            (2, 4),
            (3, 4),
            (4, 4),
            (5, 4),
            (6, 4),
            (7, 4),
            (8, 4),
            (9, 1),
        ]
        .iter()
        .map(|(value, depth)| Element {
            value: *value,
            depth: *depth,
        })
        .collect();
        assert_eq!(parse_line(x), res);
    }

    #[test]
    fn test_magnitude() {
        let lines = [
            "[[1,2],[[3,4],5]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ];
        let magnitudes = [143, 1384, 445, 791, 1137, 3488];
        for (line, m) in lines.iter().zip(magnitudes) {
            assert_eq!(magnitude(&parse_line(line)), m);
        }
    }

    #[test]
    fn test_try_explode() {
        let lines = [
            "[[[[[9,8],1],2],3],4]",
            "[7,[6,[5,[4,[3,2]]]]]",
            "[[6,[5,[4,[3,2]]]],1]",
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ];
        let results = [
            "[[[[0,9],2],3],4]",
            "[7,[6,[5,[7,0]]]]",
            "[[6,[5,[7,0]]],3]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ];
        for (line, result) in lines.iter().zip(results) {
            let mut a = parse_line(line);
            let should_explode = try_explode(&mut a);
            assert!(should_explode);
            assert_eq!(a, parse_line(result));
        }
    }

    #[test]
    fn test_try_split() {
        let mut a: Vec<Element> = [
            (0u32, 4u32),
            (7, 4),
            (4, 3),
            (7, 4),
            (8, 4),
            (0, 4),
            (13, 4),
            (1, 2),
            (1, 2),
        ]
        .iter()
        .map(|(value, depth)| Element {
            value: *value,
            depth: *depth,
        })
        .collect();
        let should_split = try_split(&mut a);
        assert!(should_split);
        assert_eq!(a, parse_line("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));
    }
}
