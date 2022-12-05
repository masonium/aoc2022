use crate::{read_lines, read_line_groups};
use std::collections::HashSet;
use std::io;

pub fn day01() -> io::Result<()> {
    let lines = read_lines("input/01.in", false)?;

    let mut v: Vec<Vec<i32>> = Vec::new();
    let mut cur_vec = Vec::new();
    for l in lines {
        if l.is_empty() {
            v.push(cur_vec.clone());
            cur_vec.clear();
        } else {
            cur_vec.push(l.parse::<i32>().unwrap());
        }
    }

    println!("{}", v.iter().map(|x| x.iter().sum::<i32>()).max().unwrap());

    let mut r: Vec<_> = v.iter().map(|x| x.iter().sum::<i32>()).collect();
    r.sort();
    r.reverse();
    let _ = r.split_off(3);
    println!("{}", r.iter().sum::<i32>());

    Ok(())
}

pub fn day02() -> io::Result<()> {
    let lines = read_lines("input/02.in", false)?;

    let mut total = 0;
    for line in &lines {
        let toks: Vec<&str> = line.split(" ").collect();
        if toks.len() < 2 {
            break;
        }
        let a = match toks[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("{}", toks[0]),
        };
        let b = match toks[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("{}", toks[1]),
        };
        let score = if a == b {
            3
        } else if (a + 1) % 3 == b % 3 {
            6
        } else {
            0
        } + b;

        total += score;
    }
    println!("{}", total);

    let mut total = 0;
    for line in &lines {
        let toks: Vec<&str> = line.split(" ").collect();
        if toks.len() < 2 {
            break;
        }
        let a = match toks[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("{}", toks[0]),
        };
        let mut b = match toks[1] {
            "X" => (a + 2) % 3,
            "Y" => a,
            "Z" => (a + 1) % 3,
            _ => panic!("{}", toks[1]),
        };
        b = if b == 0 { 3 } else { b };
        let score = if a == b {
            3
        } else if (a + 1) % 3 == b % 3 {
            6
        } else {
            0
        } + b;
        total += score;
    }
    println!("{}", total);

    Ok(())
}

pub fn day03(use_example: bool) -> io::Result<(usize, usize)> {
    let day = 3;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };
    let lines = read_lines(path, true)?;

    // Part 1
    let mut common = Vec::new();
    for line in &lines {
        let mut right = line.clone();
        let left = right.split_off(line.len() / 2);
        let x: HashSet<u8> = left.bytes().collect();
        let y: HashSet<u8> = right.bytes().collect();
        let i: HashSet<u8> = x.intersection(&y).cloned().collect();
        let c: u8 = *i.iter().next().unwrap();

        common.push(c);
    }
    let score1: usize = common
        .iter()
        .map(|x| match *x {
            y if b'a' <= y && y <= b'z' => (y - b'a' + 1) as usize,
            y if b'A' <= y && y <= b'Z' => (y - b'A' + 27) as usize,
            _ => panic!(),
        })
        .sum::<usize>();

    // Part 2
    let mut common = Vec::new();
    let mut count = 0;
    let ls: Vec<String> = lines.iter().cloned().collect();
    loop {
        if count >= ls.len() {
            break;
        }
        let right = ls[count].clone();
        count += 1;
        let mut y: HashSet<u8> = right.bytes().collect();

        let a = ls[count].clone();
        count += 1;
        y = y.intersection(&a.bytes().collect()).cloned().collect();

        let b = ls[count].clone();
        count += 1;
        y = y.intersection(&b.bytes().collect()).cloned().collect();

        common.push(y.iter().cloned().next().unwrap());
    }
    let score2: usize = common
        .iter()
        .map(|x| match *x {
            y if b'a' <= y && y <= b'z' => (y - b'a' + 1) as usize,
            y if b'A' <= y && y <= b'Z' => (y - b'A' + 27) as usize,
            _ => panic!(),
        })
        .sum::<usize>();

    Ok((score1, score2))
}

pub fn day04(use_example: bool) -> io::Result<(usize, usize)> {
    let day = 4;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let lines = read_lines(path, true)?;

    // Part 1
    let mut result1 = 0;
    for line in &lines {
        let pieces: Vec<_> = line.split(",").collect();
        let a: Vec<_> = pieces[0]
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let b: Vec<_> = pieces[1]
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if (a[0] <= b[0] && a[1] >= b[1]) || (b[0] <= a[0] && b[1] >= a[1]) {
            result1 += 1;
        }
    }
    {}

    // Part 2
    let mut result2 = 0;
    for line in lines {
        let pieces: Vec<_> = line.split(",").collect();
        let a: Vec<_> = pieces[0]
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let b: Vec<_> = pieces[1]
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if (b[1] < a[0] || a[1] < b[0]) {
            continue;
        }
        result2 += 1;
    }
    {}

    Ok((result1, result2))
}

fn day05(
    use_example: bool,
    num_stacks: usize,
    stack_height: usize,
) -> io::Result<(String, String)> {
    let day = 5;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let lines = read_lines(path, true)?;

    // Part 1
    let mut stacks: Vec<Vec<u8>> = (0..num_stacks).map(|_| Vec::new()).collect();
    for line in &lines[0..stack_height] {
        let bytes: Vec<u8> = line.bytes().collect();
        for x in 0..num_stacks {
            let c = bytes[x * 4 + 1];
            if c != b' ' {
                stacks[x].insert(0, c);
            }
        }
    }

    for line in &lines[stack_height + 1..] {
        let toks: Vec<_> = line.split(" ").collect();
        let n = toks[1].parse::<usize>().unwrap();
        let a = toks[3].parse::<usize>().unwrap() - 1;
        let b = toks[5].parse::<usize>().unwrap() - 1;

        for _ in 0..n {
            //dbg!(n, a, b, &stacks[a], &stacks[b]);
            let x = stacks[a].pop().unwrap();
            stacks[b].push(x);
        }
    }
    let r: Vec<u8> = (0..num_stacks)
        .map(|x| stacks[x][stacks[x].len() - 1])
        .collect();
    let result1 = String::from_utf8(r).unwrap();

    // Part 2
    let mut stacks: Vec<Vec<u8>> = (0..num_stacks).map(|_| Vec::new()).collect();
    for line in &lines[0..stack_height] {
        let bytes: Vec<u8> = line.bytes().collect();
        for x in 0..num_stacks {
            let c = bytes[x * 4 + 1];
            if c != b' ' {
                stacks[x].insert(0, c);
            }
        }
    }

    for line in &lines[stack_height + 1..] {
        let toks: Vec<_> = line.split(" ").collect();
        let n = toks[1].parse::<usize>().unwrap();
        let a = toks[3].parse::<usize>().unwrap() - 1;
        let b = toks[5].parse::<usize>().unwrap() - 1;

        let mut temp = Vec::new();
        for _ in 0..n {
            //dbg!(n, a, b, &stacks[a], &stacks[b]);
            let x = stacks[a].pop().unwrap();
            temp.push(x);
        }

        for _ in 0..n {
            //dbg!(n, a, b, &stacks[a], &stacks[b]);
            let x = temp.pop().unwrap();
            stacks[b].push(x);
        }
    }
    let r: Vec<u8> = (0..num_stacks)
        .map(|x| stacks[x][stacks[x].len() - 1])
        .collect();
    let result2 = String::from_utf8(r).unwrap();

    Ok((result1, result2))
}
