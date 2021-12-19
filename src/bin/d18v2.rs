use itertools::Itertools;

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

fn sum_a_b(a: &Vec<Element>, b: &Vec<Element>) -> Vec<Element> {
    let mut res = Vec::new();
    res.extend_from_slice(a);
    res.extend_from_slice(b);
    for x in res.iter_mut() {
        x.depth += 1;
    }
    while try_explode(&mut res) || try_split(&mut res) {}
    res
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
    let nums = data.lines().map(|line| parse_line(line));
    let nums2 = nums.clone();
    let result = nums
        .cartesian_product(nums2)
        .map(|(x, y)| [sum_a_b(&x, &y), sum_a_b(&y, &x)])
        .flatten()
        .map(|x| magnitude(&x))
        .max()
        .unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_a_b() {
        let tests = [
            (
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            ),
            (
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            ),
        ];
        for (l1, l2, l3) in tests.iter() {
            let a = parse_line(l1);
            let b = parse_line(l2);
            let res = parse_line(l3);
            assert_eq!(sum_a_b(&a, &b), res);
        }
    }
}
