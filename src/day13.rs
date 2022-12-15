use crate::read_line_groups;
use anyhow::Result;
use std::cmp::Ordering;

enum Item {
    Num(usize),
    List(Vec<Box<Item>>),
}

fn is_ordered(a: &Item, b: &Item) -> Ordering {
    match (a, b) {
        (Item::List(x), Item::List(y)) => {
            for i in 0..x.len() {
                if y.len() < i + 1 {
                    return Ordering::Greater;
                }

                match (&x[i].as_ref(), &y[i].as_ref()) {
                    (Item::Num(r), Item::Num(s)) => {
                        if r < s {
                            return Ordering::Less;
                        } else if r > s {
                            return Ordering::Greater;
                        }
                    }
                    (Item::Num(r), Item::List(_s)) => {
                        let new_item = Item::List(vec![Box::new(Item::Num(*r))]);
                        let ord = is_ordered(&new_item, &y[i]);
                        if ord != Ordering::Equal {
                            return ord;
                        }
                    }
                    (Item::List(_r), Item::Num(s)) => {
                        let new_item = Item::List(vec![Box::new(Item::Num(*s))]);
                        let ord = is_ordered(&x[i], &new_item);
                        if ord != Ordering::Equal {
                            return ord;
                        }
                    }
                    (Item::List(_), Item::List(_)) => {
                        let ord = is_ordered(&x[i], &y[i]);
                        if ord != Ordering::Equal {
                            return ord;
                        }
                    }
                }
            }
            if x.len() != y.len() {
                return Ordering::Less;
            }
            return Ordering::Equal;
        }
        _ => {
            panic!();
        }
    }
}

fn parse_list(s: &[u8], start: usize) -> (Item, usize) {
    let mut items = vec![];
    let mut i = start;
    while i < s.len() {
        match s[i] {
            b',' => {
                i += 1;
                continue;
            }
            b']' => {
                return (Item::List(items), i + 1);
            }
            b'0'..=b'9' => {
                let mut c: usize = 0;
                while (b'0'..=b'9').contains(&s[i]) {
                    c = c * 10 + s[i] as usize;
                    i += 1;
                }
                items.push(Box::new(Item::Num(c)));
            }
            b'[' => {
                let (x, n) = parse_list(s, i + 1);
                items.push(Box::new(x));
                i = n;
            }
            _ => {
                panic!();
            }
        }
    }
    (Item::List(items), i)
}

pub fn day13(use_example: bool) -> Result<(usize, usize)> {
    let day = 13;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let groups = read_line_groups(path)?;

    // Part 1
    let mut sum = 0;
    for (i, group) in groups.iter().enumerate() {
        let l1 = group[0].clone();
        let l2 = group[1].clone();

        let a = parse_list(l1.as_bytes(), 1).0;
        let b = parse_list(l2.as_bytes(), 1).0;

        if is_ordered(&a, &b) == Ordering::Less {
            sum += i + 1;
        }
    }

    let result1 = sum;
    {}

    let mut all = vec![];
    let mut x = 1;
    for group in &groups {
        let l1 = group[0].clone();
        let l2 = group[1].clone();

        let a = parse_list(l1.as_bytes(), 1).0;
        let b = parse_list(l2.as_bytes(), 1).0;
        all.push((a, x));
        x += 1;
        all.push((b, x));
        x += 1;
    }

    let s = x;
    all.push((parse_list("[[2]]".as_bytes(), 1).0, x));
    x += 1;
    let e = x;
    all.push((parse_list("[[6]]".as_bytes(), 1).0, x));

    all.sort_by(|x, y| is_ordered(&x.0, &y.0));

    let a = all.iter().position(|x| x.1 == s).unwrap() + 1;
    let b = all.iter().position(|x| x.1 == e).unwrap() + 1;

    // Part 2
    let result2 = a * b;
    {}

    Ok((result1, result2))
}
