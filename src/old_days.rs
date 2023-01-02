use crate::string::*;
use crate::*;
use anyhow::Result;
use ndarray::{Array2, Array3};
use std::collections::{HashMap, HashSet};
use std::collections::LinkedList;
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
        let toks: Vec<&str> = line.split(' ').collect();
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
        let toks: Vec<&str> = line.split(' ').collect();
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
            y if (b'a'..=b'z').contains(&y) => (y - b'a' + 1) as usize,
            y if (b'A'..=b'Z').contains(&y) => (y - b'A' + 27) as usize,
            _ => panic!(),
        })
        .sum::<usize>();

    // Part 2
    let mut common = Vec::new();
    let mut count = 0;
    let ls: Vec<String> = lines.to_vec();
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
            y if (b'a'..=b'z').contains(&y) => (y - b'a' + 1) as usize,
            y if (b'A'..=b'Z').contains(&y) => (y - b'A' + 27) as usize,
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
        let pieces: Vec<_> = line.split(',').collect();
        let a: Vec<_> = pieces[0]
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let b: Vec<_> = pieces[1]
            .split('-')
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
        let pieces: Vec<_> = line.split(',').collect();
        let a: Vec<_> = pieces[0]
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let b: Vec<_> = pieces[1]
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if b[1] < a[0] || a[1] < b[0] {
            continue;
        }
        result2 += 1;
    }
    {}

    Ok((result1, result2))
}

pub fn day05(
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
        let toks: Vec<_> = line.split(' ').collect();
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
        let toks: Vec<_> = line.split(' ').collect();
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

struct File {
    size: usize,
}

struct Dir {
    name: String,
    parent: String,
    files: Vec<File>,
    children: Vec<String>,
}

impl Default for Dir {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            parent: "".to_string(),
            files: vec![],
            children: vec![],
        }
    }
}

impl Dir {
    fn size(&self, dirs: &HashMap<String, Dir>) -> usize {
        let f = self.files.iter().map(|x| x.size).sum::<usize>();
        let c = self
            .children
            .iter()
            .map(|x| dirs[x].size(dirs))
            .sum::<usize>();
        f + c
    }
}

pub fn day07(use_example: bool) -> Result<(usize, usize)> {
    let day = 7;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let lines = read_lines(path, true)?;
    let mut dirs: HashMap<String, Dir> = HashMap::new();

    let mut curr: String = "/".to_string();
    let root = Dir {
        name: "/".to_string(),
        ..Default::default()
    };
    dirs.insert("/".to_string(), root);

    for line in &lines {
        let toks: Vec<_> = line.split(' ').collect();
        if line.starts_with('$') {
            if toks[1] == "cd" {
                if toks[2] == ".." {
                    let d = dirs[&curr].parent.clone();
                    curr = d;
                } else {
                    curr = format!("{};{}", curr, toks[2]);
                }
            } else if toks[1] == "ls" {
            }
        } else if line.starts_with("dir") {
            let t = toks[1].to_string();
            let subname = format!("{};{}", curr, t);
            dirs.entry(subname.clone()).or_default().parent = curr.clone();
            dirs.entry(subname.clone()).or_default().name = subname.clone();
            dirs.entry(curr.clone())
                .or_default()
                .children
                .push(subname.clone());
        } else {
            let size = as_usize(toks[0]);
            dirs.entry(curr.clone())
                .or_default()
                .files
                .push(File { size });
        }
    }

    // Part 1
    let result1: usize = dirs
        .values()
        .map(|x| x.size(&dirs))
        .filter(|z| *z <= 100000)
        .sum();

    // Part 2
    let size: isize = dirs["/;/"].size(&dirs) as isize;
    println!("{}", size);
    let min_d = size - 40000000;
    println!("{}", min_d);
    let result2 = dirs
        .values()
        .map(|x| x.size(&dirs))
        .filter(|z| *z >= min_d as usize)
        .min()
        .unwrap();
    {}

    Ok((result1, result2))
}

pub fn day08(use_example: bool) -> Result<(usize, usize)> {
    let day = 8;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let g = read_digit_grid(path);
    let dim = g.dim();

    // Part 1
    let mut count = 0;
    for j in 0..dim.1 {
        for i in 0..dim.0 {
            let t = g[(i, j)];
            let mut found = true;
            for x in 0..i as isize {
                if g[(x as usize, j)] >= t {
                    found = false;
                    break;
                }
            }
            if found {
                count += 1;
                continue;
            }
            found = true;
            for x in i + 1..dim.0 {
                if g[(x, j)] >= t {
                    found = false;
                    break;
                }
            }
            if found {
                count += 1;
                continue;
            }
            found = true;
            for y in 0..j as isize {
                if g[(i, y as usize)] >= t {
                    found = false;
                    break;
                }
            }
            if found {
                count += 1;
                continue;
            }
            found = true;
            for y in j + 1..dim.1 {
                if g[(i, y)] >= t {
                    found = false;
                    break;
                }
            }
            if found {
                count += 1;
                continue;
            }
        }
    }
    let result1 = count;

    // Part 2
    let mut result2 = 0;
    for j in 1..dim.1 - 1 {
        for i in 1..dim.0 - 1 {
            let t = g[(i, j)];
            //	    let mut found = true;
            let mut total = 1;

            let mut c = 0;
            for x in (0..i as isize).rev() {
                c += 1;
                if g[(x as usize, j)] >= t {
                    break;
                }
            }
            total *= c;
            c = 0;
            for x in i + 1..dim.0 {
                c += 1;
                if g[(x, j)] >= t {
                    break;
                }
            }
            total *= c;
            c = 0;
            for y in (0..j as isize).rev() {
                c += 1;
                if g[(i, y as usize)] >= t {
                    break;
                }
            }
            total *= c;
            c = 0;
            for y in j + 1..dim.1 {
                c += 1;
                if g[(i, y)] >= t {
                    break;
                }
            }
            total *= c;
            result2 = result2.max(total);
        }
    }

    Ok((result1, result2))
}

fn add(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn next_to(a: (isize, isize), b: (isize, isize)) -> bool {
    (a.0 - b.0).abs().max((a.1 - b.1).abs()) <= 1
}

pub fn day09(use_example: bool) -> Result<(usize, usize)> {
    let day = 9;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let lines = read_lines(path, true)?;

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut positions: HashSet<(isize, isize)> = HashSet::new();

    // Part 1
    let mut dirs = HashMap::new();
    dirs.insert(b'U', (0, 1));
    dirs.insert(b'D', (0, -1));
    dirs.insert(b'R', (1, 0));
    dirs.insert(b'L', (-1, 0));

    positions.insert(tail);

    for line in &lines {
        let toks: Vec<&str> = line.split(' ').collect();
        let d = toks[0].bytes().next().unwrap();
        let dir = dirs[&d];
        let num = as_usize(toks[1]);

        for _ in 0..num {
            head = add(head, dir);

            if next_to(head, tail) {
                continue;
            }

            let mut dx = head.0 - tail.0;
            dx = dx.abs().min(1) * dx.signum();
            let mut dy = head.1 - tail.1;
            dy = dy.abs().min(1) * dy.signum();
            tail = (tail.0 + dx, tail.1 + dy);
            positions.insert(tail);
        }
    }
    let result1 = positions.len();

    // Part 2
    let mut knots: Vec<_> = vec![(0, 0); 10];
    positions.clear();

    for line in &lines {
        let toks: Vec<&str> = line.split(' ').collect();
        let d = toks[0].bytes().next().unwrap();
        let dir = dirs[&d];
        let num = as_usize(toks[1]);

        for _ in 0..num {
            knots[0] = add(knots[0], dir);
            for i in 1..10 {
                if next_to(knots[i - 1], knots[i]) {
                    continue;
                }

                let mut dx = knots[i - 1].0 - knots[i].0;
                dx = dx.abs().min(1) * dx.signum();
                let mut dy = knots[i - 1].1 - knots[i].1;
                dy = dy.abs().min(1) * dy.signum();
                knots[i] = (knots[i].0 + dx, knots[i].1 + dy);
            }
            positions.insert(knots[9]);
        }
    }

    let result2 = positions.len();
    Ok((result1, result2))
}

pub fn day10(use_example: bool) -> Result<(isize, isize)> {
    let day = 10;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };
    let signals = vec![20, 60, 100, 140, 180, 220];

    let lines = read_lines(path, true)?;

    // Part 1
    let mut n = 0;
    let mut cycle = 0;
    let mut result1 = 0;
    let mut reg = 1;
    for line in &lines {
        let tok: Vec<_> = line.split(' ').collect();
        let mut new_reg = reg;
        if tok[0] == "noop" {
            cycle += 1;
        } else {
            let n = as_isize(tok[1]);
            new_reg = reg + n;
            cycle += 2;
        }
        if cycle >= signals[n] {
            result1 += reg * signals[n];
            n += 1;
            if n >= signals.len() {
                break;
            }
        }
        reg = new_reg
    }

    // Part 2
    let row = vec!['.'; 40];

    let mut grid = vec![];
    for _ in 0..6 {
        grid.push(row.clone());
    }
    let mut pixel_pos: [isize; 2] = [0, 0];
    let mut sprite_pos: isize = 1;
    for line in &lines {
        let tok: Vec<_> = line.split(' ').collect();
        if (sprite_pos - pixel_pos[0]).abs() <= 1 {
            grid[pixel_pos[1] as usize][pixel_pos[0] as usize] = '#';
        }
        pixel_pos[0] += 1;
        if pixel_pos[0] == 40 {
            pixel_pos[1] += 1;
            pixel_pos[0] = 0;
            if pixel_pos[1] == 6 {
                break;
            }
        }
        if tok[0] == "addx" {
            if (sprite_pos - pixel_pos[0]).abs() <= 1 {
                grid[pixel_pos[1] as usize][pixel_pos[0] as usize] = '#';
            }

            pixel_pos[0] += 1;
            if pixel_pos[0] == 40 {
                pixel_pos[1] += 1;
                pixel_pos[0] = 0;
                if pixel_pos[1] == 6 {
                    break;
                }
            }
            sprite_pos += as_isize(tok[1]);
        }
    }
    for g in grid.iter() {
        for c in g.iter() {
            print!("{}", c);
        }
        println!();
    }

    Ok((result1, 0))
}

#[allow(unused)]
fn day12(use_example: bool) -> Result<(usize, usize)> {
    let day = 12;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let mut grid = read_grid(path);

    let mut start = (0, 0);
    let mut end = (0, 0);

    let d = grid.dim();
    dbg!(d);
    for i in 0..d.0 {
        for j in 0..d.1 {
            if grid[(i, j)] == b'S' {
                start = (i, j);
                grid[(i, j)] = b'a';
            } else if grid[(i, j)] == b'E' {
                end = (i, j);
                grid[(i, j)] = b'z';
            }
        }
    }
    type Pos = (usize, usize);
    let mut visited: HashSet<_> = HashSet::new();
    let mut s = vec![(start, 0)];
    let mut result = 0;
    while !s.is_empty() {
        let (top, dist) = s.pop().unwrap();
        //dbg!(top);
        if visited.contains(&top) {
            continue;
        }
        visited.insert(top);

        for dir in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let new_pos = (top.0 as isize + dir.0, top.1 as isize + dir.1);
            //dbg!(new_pos);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= d.0 as isize
                || new_pos.1 >= d.1 as isize
            {
                continue;
            }
            let np = (new_pos.0 as usize, new_pos.1 as usize);
            if grid[np] > grid[top] + 1 {
                continue;
            }

            if np == end {
                result = dist + 1;
                break;
            }
            //println!("inserting {:?}", np);
            s.insert(0, (np, dist + 1));
        }
    }

    // Part 1
    let mut result1 = result;
    {}

    let mut result = d.0 * d.1;
    for i in 0..d.0 {
        for j in 0..d.1 {
            if grid[(i, j)] != b'a' {
                continue;
            }
            let start = (i, j);
            let mut visited: HashSet<_> = HashSet::new();
            let mut s = vec![(start, 0)];
            while !s.is_empty() {
                let (top, dist) = s.pop().unwrap();
                //dbg!(top);
                if visited.contains(&top) {
                    continue;
                }
                visited.insert(top);

                for dir in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
                    let new_pos = (top.0 as isize + dir.0, top.1 as isize + dir.1);
                    //dbg!(new_pos);
                    if new_pos.0 < 0
                        || new_pos.1 < 0
                        || new_pos.0 >= d.0 as isize
                        || new_pos.1 >= d.1 as isize
                    {
                        continue;
                    }
                    let np = (new_pos.0 as usize, new_pos.1 as usize);
                    if grid[np] > grid[top] + 1 {
                        continue;
                    }

                    if np == end {
                        result = result.min(dist + 1);
                        break;
                    }
                    //println!("inserting {:?}", np);
                    s.insert(0, (np, dist + 1));
                }
            }
        }
    }
    // Part 2
    let mut result2 = result;
    {}

    Ok((result1, result2))
}

#[allow(unused)]
fn day14(use_example: bool) -> Result<(usize, usize)> {
    let day = 14;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let lines = read_lines(path, true)?;
    let mut result1 = 0;
    {
        let mut arr = Array2::zeros((601, 601));

        // Part 1
        for line in &lines {
            let points: Vec<(usize, usize)> = line
                .split(" -> ")
                .map(|x| {
                    let t: Vec<_> = x.split(",").map(as_usize).collect();
                    (t[0], t[1])
                })
                .collect();
            //	dbg!(&points);

            for i in 0..points.len() - 1 {
                let p0 = points[i];
                let p1 = points[i + 1];
                if p0.0 == p1.0 {
                    let a = p0.1.min(p1.1);
                    let b = p0.1.max(p1.1);
                    for j in a..=b {
                        arr[(p0.0, j)] = 1;
                    }
                } else {
                    let a = p0.0.min(p1.0);
                    let b = p0.0.max(p1.0);
                    for j in a..=b {
                        arr[(j, p0.1)] = 1;
                    }
                }
            }
        }
        println!("{}", arr.iter().sum::<usize>());
        println!("{}", arr[(500, 9)]);
        let mut count = 0;
        let mut land = true;

        while land {
            let mut grain = (500, 0);
            land = false;
            while grain.1 < 500 {
                //println!("{} {}", grain.0, grain.1);
                let new = (grain.0, grain.1 + 1);
                if arr[new] == 0 {
                    //		arr[new] = 1;
                    grain = new;
                    continue;
                }
                let new = (grain.0 - 1, grain.1 + 1);
                if arr[new] == 0 {
                    //		arr[new] = 1;
                    grain = new;
                    continue;
                }
                let new = (grain.0 + 1, grain.1 + 1);
                if arr[new] == 0 {
                    //		arr[new] = 1;
                    grain = new;
                    continue;
                }
                arr[grain] = 1;
                land = true;
                break;
            }
            if land {
                count += 1;
            }
        }

        result1 = count;
    }
    // Part 2
    let mut result2 = 0;

    {
        let mut arr = Array2::zeros((1201, 601));

        // Part 1
        let mut result1 = 0;
        let mut max_y = 0;
        for line in &lines {
            let points: Vec<(usize, usize)> = line
                .split(" -> ")
                .map(|x| {
                    let t: Vec<_> = x.split(",").map(as_usize).collect();
                    (t[0], t[1])
                })
                .collect();
            //	dbg!(&points);

            for i in 0..points.len() - 1 {
                let p0 = points[i];
                let p1 = points[i + 1];
                max_y = max_y.max(p0.1);
                max_y = max_y.max(p1.1);
                if p0.0 == p1.0 {
                    let a = p0.1.min(p1.1);
                    let b = p0.1.max(p1.1);
                    for j in a..=b {
                        arr[(p0.0, j)] = 1;
                    }
                } else {
                    let a = p0.0.min(p1.0);
                    let b = p0.0.max(p1.0);
                    for j in a..=b {
                        arr[(j, p0.1)] = 1;
                    }
                }
            }
        }
        println!("{}", arr.iter().sum::<usize>());
        for x in 0..1201 {
            arr[(x, max_y + 2)] = 1;
        }
        let mut count = 0;
        let mut land = true;

        while land {
            let mut grain = (500, 0);
            land = false;
            while grain.1 < 500 {
                //println!("{} {}", grain.0, grain.1);
                let new = (grain.0, grain.1 + 1);
                if arr[new] == 0 {
                    //		arr[new] = 1;
                    grain = new;
                    continue;
                }
                let new = (grain.0 - 1, grain.1 + 1);
                if arr[new] == 0 {
                    //		arr[new] = 1;
                    grain = new;
                    continue;
                }
                let new = (grain.0 + 1, grain.1 + 1);
                if arr[new] == 0 {
                    //		arr[new] = 1;
                    grain = new;
                    continue;
                }
                arr[grain] = 1;
                land = true;
                break;
            }
            if land {
                count += 1;
            }
            if grain == (500, 0) {
                break;
            }
        }
        result2 = count;
    }

    Ok((result1, result2))
}

struct Intervals {
    v: Vec<(isize, isize)>,
}
impl Intervals {
    fn new(s: isize) -> Intervals {
        Intervals { v: vec![(0, s)] }
    }

    fn isect(&mut self, a: isize, b: isize) {
        let mut i = 0;

        while i < self.v.len() {
            let x = self.v[i];
            if a <= x.0 && x.1 <= b {
                self.v.remove(i);
            } else if a <= x.0 && b >= x.0 {
                self.v[i] = (b + 1, x.1);
                i += 1;
            } else if a <= x.1 && b >= x.1 {
                self.v[i] = (x.0, a - 1);
                i += 1;
            } else if a > x.0 && b < x.1 {
                self.v[i] = (x.0, a - 1);
                self.v.insert(i + 1, (b + 1, x.1));
                break;
            } else {
                i += 1;
            }
        }
    }
    fn count(&self) -> usize {
        self.v
            .iter()
            .map(|(a, b)| (b - a + 1) as usize)
            .sum::<usize>()
    }
}

#[allow(unused)]
fn day15(use_example: bool, row: isize, row_max: isize) -> Result<(usize, usize)> {
    let day = 15;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let lines = read_lines(path, true)?;

    let mut s = vec![];
    for line in &lines {
        // let toks: Vec<_> = line.split(" ").collect();

        // let a = as_isize(toks[2].strip_prefix("x=").unwrap().strip_suffix(",").unwrap());
        // let b = as_isize(toks[3].strip_prefix("y=").unwrap().strip_suffix(":").unwrap());
        // let c = as_isize(toks[8].strip_prefix("x=").unwrap().strip_suffix(",").unwrap());
        // let d = as_isize(toks[9].strip_prefix("y=").unwrap().strip_suffix("").unwrap());

        let nums = extract_ints(&line);
        s.push(((nums[0], nums[1]), (nums[2], nums[3])));
    }

    let mut min = 100000000;
    let mut max = -100000000;

    for r in &s {
        let dist = (r.0 .0 - r.1 .0).abs() as usize + (r.0 .1 - r.1 .1).abs() as usize;

        let x_dist = dist as isize - (row - r.0 .1).abs();
        if x_dist >= 0 {
            min = min.min(r.0 .0 - x_dist);
            max = max.max(r.0 .0 + x_dist);
        }
    }
    dbg!(min, max);

    let mut reach = vec![0; (max - min + 1) as usize];

    for r in &s {
        let dist = (r.0 .0 - r.1 .0).abs() as usize + (r.0 .1 - r.1 .1).abs() as usize;

        let x_dist = dist as isize - (row - r.0 .1).abs();
        if x_dist >= 0 {
            for i in (r.0 .0 - x_dist)..=(r.0 .0 + x_dist) {
                reach[(i - min) as usize] = 1;
            }
        }
    }
    for r in &s {
        if r.1 .1 == row {
            if min <= r.1 .0 && r.1 .0 <= max {
                reach[(row - min + 1) as usize] = 0;
            }
        }
    }

    // Part 1
    let mut result1 = reach.iter().sum();
    {}
    let mut result2 = 0;
    let mut ivals: Vec<_> = (0..=row_max).map(|_| Intervals::new(row_max)).collect();
    let min = 0;
    let max = row_max;
    let mut reach = vec![0; (max - min + 1) as usize];
    let mut ival = Intervals::new(row_max);

    for r in &s {
        let dist = (r.0 .0 - r.1 .0).abs() + (r.0 .1 - r.1 .1).abs();
        if 0 <= r.1 .1 && r.1 .1 <= row_max {
            ivals[r.1 .1 as usize].isect(r.1 .0, r.1 .0);
        }
        if 0 <= r.0 .1 && r.0 .1 <= row_max {
            ivals[r.0 .1 as usize].isect(r.0 .0, r.0 .0);
        }
        for d in -dist..=dist {
            let y = r.0 .1 + d;
            if y < 0 || y > row_max {
                continue;
            }
            let xd = dist - d.abs();
            ivals[y as usize].isect(r.0 .0 - xd, r.0 .0 + xd);
        }
    }

    for (i, iv) in ivals.iter().enumerate() {
        if iv.count() != 0 {
            result2 = iv.v[0].0 as usize * 4000000 as usize + i as usize;
        }
    }

    // Part 1

    // Part 2
    {}

    Ok((result1, result2))
}

#[allow(unused)]
fn day17(use_example: bool) -> Result<(usize, usize)> {
    let day = 17;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let pieces_lines = read_line_groups("input/pieces.txt")?;

    type Piece = Vec<(isize, isize)>;
    let mut pieces: Vec<Piece> = vec![];
    for piece in pieces_lines {
        let mut p = vec![];
        for i in 0..piece.len() {
            for (j, b) in piece[i].bytes().enumerate() {
                if b == b'#' {
                    p.push(((piece.len() - i - 1) as isize, j as isize));
                }
            }
        }
        pieces.push(p);
    }

    let dirs: Vec<_> = read_lines(path, true)?[0].bytes().collect();
    let mut d = 0;

    fn try_dir(p: &Piece, d: &(isize, isize), c: &Array2<usize>) -> Option<Piece> {
        let new_piece: Piece = p.iter().map(|x| (x.0 + d.0, x.1 + d.1)).collect();
        //dbg!(&new_piece);
        for x in &new_piece {
            if x.0 < 0 {
                return None;
            }
            if x.1 < 0 || x.1 >= 7 {
                return None;
            }
            if c[(x.0 as usize, x.1 as usize)] == 1 {
                return None;
            }
        }
        return Some(new_piece);
    }

    let mut chamber = Array2::zeros((dirs.len() * 5 * 3 * 700, 7));

    let mut max_height = -1;
    let mut d = 0;
    let mut ans4 = 0;
    let mut ans6 = 0;
    let mut ans2022 = 0;
    let period = 2_i32.pow(21) as usize;
    let mut seen_top3: HashMap<(usize, usize, usize), (usize, usize)> = HashMap::new();

    println!("{}", dirs.len());

    let mut l = 0;
    let mut last = 0;
    let s = loop {
        let r = l % 5;
        l += 1;
        if l % (dirs.len() * 5) == 1 {
            println!("max: {}", max_height + 1 - last);
            last = max_height + 1;
        }
        let mut piece = pieces[r].clone();

        // place the piece
        piece = try_dir(&piece, &(max_height + 4, 2), &chamber).unwrap();
        //	println!("{:?}", &piece);

        loop {
            // move dir
            let dir = if dirs[d] == b'<' { (0, -1) } else { (0, 1) };
            //	    println!("{}", dirs[d] as char);

            piece = try_dir(&piece, &dir, &chamber).unwrap_or(piece);
            //	    println!("{:?}", &piece);
            d = (d + 1) % dirs.len();

            if let Some(p) = try_dir(&piece, &(-1, 0), &chamber) {
                piece = p;
            //		println!("{:?}", &piece);
            } else {
                for x in piece {
                    chamber[(x.0 as usize, x.1 as usize)] = 1;
                    max_height = max_height.max(x.0);
                }
                break;
            }
        }
        // for i in (0..=max_height).rev() {
        //     for j in 0.. 7 {
        // 	if chamber[(i as usize, j as usize)] == 1 {
        // 	    print!("#");
        // 	} else {
        // 	    print!(".");
        // 	}
        //     }
        //     println!();
        // }
        // println!("*******");

        if l == 2022 {
            ans2022 = max_height + 1;
        }
        let mut c = 0;
        let mut top = 0;
        for i in max_height - 7..=max_height {
            for j in 0..7 {
                if i < 0 || chamber[(i as usize, j as usize)] == 1 {
                    top = top | 1 << c;
                }
                c += 1;
            }
        }
        let state = (top, r, d);
        // println!("{}", max_height);
        if seen_top3.contains_key(&state) {
            break state;
        } else {
            seen_top3.insert(state, (l, max_height as usize));
        }
    };

    // Part 1
    let old_top = seen_top3[&s];
    let old_iter = old_top.0;
    let old_max = old_top.1;
    let upto = (2022 - old_iter) / (l - old_iter) * (max_height as usize - old_max) + old_max;
    let rem = 2022 - (old_iter + ((2022 - old_iter) / (l - old_iter)) * (l - old_iter));
    let x = seen_top3
        .iter()
        .find(|x| x.1 .0 == old_iter + rem)
        .map(|x| x.1 .1)
        .unwrap();
    println!("{}", x - old_max + upto);
    let mut result1 = (x - old_max + upto + 1) as usize;
    {}

    // Part 2
    let period = 1000000000000usize;
    let old_top = seen_top3[&s];
    let old_iter = old_top.0;
    let old_max = old_top.1;
    dbg!(old_iter, old_max, l, max_height);
    let upto = (period - old_iter) / (l - old_iter) * (max_height as usize - old_max) + old_max;
    let rem = period - (old_iter + (period - old_iter) / (l - old_iter) * (l - old_iter));
    println!("{}", rem);
    //let rem = period -
    let x = seen_top3
        .iter()
        .find(|x| x.1 .0 == old_iter + rem)
        .map(|x| x.1 .1)
        .unwrap();
    //println!("{}", x - old_max + upto);
    let mut result2 = (x - old_max + upto + 1) as usize;
    {}

    Ok((result1, result2))
}

#[allow(unused)]
fn day18(use_example: bool) -> Result<(usize, usize)> {
    let day = 18;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let lines = read_lines(path, true)?;

    let points: HashSet<_> = lines
        .iter()
        .map(|x| {
            let a = extract_uints(x);
            (a[0] + 1, a[1] + 1, a[2] + 1)
        })
        .collect();

    // Part 1
    let mut num_faces = 0;
    for p in &points {
        for d in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let np = (p.0 as isize + d.0, p.1 as isize + d.1, p.2 as isize + d.2);
            let npu = (np.0 as usize, np.1 as usize, np.2 as usize);
            if !points.contains(&npu) {
                num_faces += 1;
            }
        }
    }
    let mut result1 = num_faces;

    const CUBE: usize = 1;
    const STEAM: usize = 2;
    let n = 24;
    let mut vals = Array3::zeros((24, 24, 24));
    for p in &points {
        vals[*p] = CUBE;
    }
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                if x == 0 || y == 0 || z == 0 || x == n - 1 || y == n - 1 || z == n - 1 {
                    vals[(x, y, z)] = STEAM; // outside
                }
            }
        }
    }

    loop {
        let mut new = false;
        for x in 1..23 {
            for y in 1..23 {
                for z in 1..23 {
                    if vals[(x, y, z)] == 1 || vals[(x, y, z)] == 2 {
                        continue;
                    }

                    if vals[(x - 1, y, z)] == 2
                        || vals[(x + 1, y, z)] == 2
                        || vals[(x, y - 1, z)] == 2
                        || vals[(x, y + 1, z)] == 2
                        || vals[(x, y, z - 1)] == 2
                        || vals[(x, y, z + 1)] == 2
                    {
                        new = true;
                        vals[(x, y, z)] = 2;
                    }
                }
            }
        }
        if !new {
            break;
        }
    }

    let mut num_faces = 0;
    for p in &points {
        for d in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let np = (p.0 as isize + d.0, p.1 as isize + d.1, p.2 as isize + d.2);
            let npu = (np.0 as usize, np.1 as usize, np.2 as usize);
            if vals[npu] == STEAM {
                num_faces += 1;
            }
        }
    }

    // Part 2
    let mut result2 = num_faces;

    Ok((result1, result2))
}

fn mix(r: &mut LinkedList<(usize, isize)>) {
   let mut i = 0;
    let n = r.len();
    let ni = n as isize;
    while i < n {
	let mut x = r.pop_front().unwrap();
	while x.0 != i {
	    // insert on the end
	    r.push_back(x);
	    x = r.pop_front().unwrap();
	    continue;
	}

	if x.1 > 0 {
	    // println!("{}", x.1 );
	    let mut tail = r.split_off(x.1 as usize % (n - 1));
	    tail.push_front(x);
	    r.append(&mut tail);
	} else if x.1 < 0 {
	    let split = ((n as isize + x.1 - 1) % (ni -1) + (ni - 1)) % (ni - 1);
	    // println!("{split}");
	    let mut tail = r.split_off(split as usize);
	    tail.push_front(x);
	    r.append(&mut tail);
	} else {
	    r.push_back(x);
	}
	i += 1;
	//let v: Vec<_> = r.iter().map(|x| x.1).collect();
	//println!("{:?}", &v);

    }

}

pub fn day20(use_example: bool) -> Result<(isize, isize)> {
    let day = 20;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let lines = read_lines(path, true)?;

    // Part 1
    let nums: Vec<_> = lines.iter().map(|x| as_isize(&x)).enumerate().collect();
    let n = nums.len();
    let mut r = LinkedList::from_iter(nums.iter().cloned());
    
    mix(&mut r);

    let v: Vec<_> = r.into_iter().collect();
    let p = v.iter().position(|x| x.1 == 0).unwrap();
    let result1 = v[(p + 1000) % n].1 + v[(p + 2000) % n].1 + v[(p + 3000) % n].1;

    // Part 2
    let decrypt: Vec<_> = nums.iter().map(|x| (x.0, x.1 * 811589153)).collect();
    
    let mut s = LinkedList::from_iter(decrypt);
    for _ in 0..10 {
	mix(&mut s);
    }
    let v: Vec<_> = s.into_iter().collect();
    let p = v.iter().position(|x| x.1 == 0).unwrap();
    let result2 = v[(p + 1000) % n].1 + v[(p + 2000) % n].1 + v[(p + 3000) % n].1;

    Ok((result1, result2))
}

pub fn day24(use_example: bool) -> Result<(usize, usize)> {
    let day = 24;
    let path = if use_example {
	format!("input/example{:02}.in", day)
    } else {
	format!("input/{:02}.in", day)
    };

    let g = read_grid(path);
    let dim = g.dim();
    let mut blizzards = HashMap::new();
    let mut walls = HashSet::new();
    for i in 0..dim.0 {
	for j in 0..dim.1 {
	    if [b'<', b'>', b'^', b'v'].contains(&g[(i, j)]) {
		blizzards.insert((i, j), g[(i, j)] as char);
	    } else if g[(i, j)] == b'#' {
		walls.insert((i as isize, j as isize));
	    }
	}
    }
    walls.insert((-1, 1));
    walls.insert((dim.0 as isize, dim.1 as isize-2));

    let start_pos = (0 as isize, 1 as isize, 0 as usize);
    let end_pos: (isize, isize) = (dim.0 as isize - 1, dim.1 as isize -2);

    let mut dirs = HashMap::new();
    let mut seen = HashSet::new();
    dirs.insert('<', (0, -1));
    dirs.insert('>', (0, 1));
    dirs.insert('^', (-1, 0));
    dirs.insert('v', (1, 0));
    let mut ds: Vec<_> = dirs.values().cloned().collect();
    ds.push((0, 0));

    let mut closed_list = HashMap::new();

    fn blizzard_at_step(bx: isize, by: isize, b: char, t : usize, dx: usize, dy: usize) -> (isize, isize) {
	let adjx = bx - 1;
	let adjy = by - 1;
	let dd = match b {
	    '<' => (0, -1),
	    '>' => (0, 1),
	    '^' => (-1, 0),
	    'v' => (1, 0),
	    _ => panic!()
	};
	let mut bnx = (adjx + dd.0 * t as isize) % (dx - 2) as isize;
	let mut bny = (adjy + dd.1 * t as isize) % (dy - 2) as isize;
	while bnx < 0 {
	    bnx += dx  as isize- 2;
	}
	while bny < 0 {
	    bny += dy as isize- 2;
	}
	(bnx + 1, bny + 1)
    }

    let mut pq = priority_queue::PriorityQueue::with_capacity(2000);
    pq.push(start_pos, -(0 + (start_pos.0 as isize - end_pos.0 as isize).abs() + (start_pos.1 as isize - end_pos.1 as isize).abs()) );
    let result1;
    let mut result2 = 0;
    loop {
	let a @ ((px, py, mut t), _val) = pq.pop().unwrap();
	if (px, py) == end_pos {
	    result1 = t;
	    break;
	}

	// consider each direction {
	for d in &ds {
	    let nx = px as isize + d.0;
	    let ny = py as isize + d.1;
	    if walls.contains(&(nx , ny)) {
		continue;
	    }

	    // check for blizzards at t +1
	    let mut clear = true;
	    
	    for ((bx, by), b) in blizzards.iter() {
		let (btx, bty) = blizzard_at_step(*bx as isize, *by as isize, *b, t + 1, dim.0, dim.1);
		if btx == nx && bty == ny {
		    clear = false;
		    break;
		}
	    };
	    if !clear {
		continue;
	    }
	    pq.push((nx as isize, ny as isize, t+1), -(t as isize + 1 + (nx - end_pos.0).abs() + (ny - end_pos.1).abs()));
	}
	let mut z = closed_list.entry((px, py)).or_insert(t);
	*z = *z.min(&mut t);
    }
    println!("{}", pq.len());
    pq.clear();
    pq.push((end_pos.0, end_pos.1, result1), -(result1 as isize + (start_pos.0 as isize - end_pos.0 as isize).abs() + (start_pos.1 as isize - end_pos.1 as isize).abs()) );
    
    loop {
//	println!("{}", pq.len());
//	pq.iter().for_each(|x| println!("{:?}", x));
	let a @ ((px, py, mut t), _val) = pq.pop().unwrap();
	if seen.contains(&a) {
	    continue;
	}
	seen.insert(a);
//	println!("{:?}", (px, py, t, val));
	if (px, py) == (start_pos.0, start_pos.1) {
	    result2 = t;
	    break;
	}

	// consider each direction {
	for d in &ds {
	    let nx = px as isize + d.0;
	    let ny = py as isize + d.1;
	    if walls.contains(&(nx , ny)) {
//		println!("skipping wall at {:?} (t={})", (nx, ny), t+1);
		continue;
	    }

	    // check for blizzards at t +1
//	    println!("checking ({}, {})", nx, ny);
	    let mut clear = true;
	    
	    for ((bx, by), b) in blizzards.iter() {
		let (btx, bty) = blizzard_at_step(*bx as isize, *by as isize, *b, t + 1, dim.0, dim.1);
//		println!("{:?}, {}  at t={} => {:?}", (bx, by), b, t+1, (btx, bty));
		if btx == nx && bty == ny {
		    clear = false;
//		    println!("skipping tornado at {:?} (t={})", (btx, bty), t+1);
		    break;
		}
	    };
	    if !clear {
		continue;
	    }
	    let new = (nx as isize, ny as isize, t+1);
	    let f = -(t as isize + 1 + (nx - start_pos.0).abs() + (ny - start_pos.1).abs());
	    if closed_list.get(&(nx, ny)).unwrap_or(&new.2) < &new.2 {
		continue;
	    }
	    pq.push(new, f);
	}

	let mut z = closed_list.entry((px, py)).or_insert(t);
	*z = *z.min(&mut t);
    }
    println!("{}", result2);
    pq.clear();
    pq.push((start_pos.0, start_pos.1, result2), -(result2 as isize + (start_pos.0 as isize - end_pos.0 as isize).abs() + (start_pos.1 as isize - end_pos.1 as isize).abs()) );

    loop {
	let a @ ((px, py, mut t), _val) = pq.pop().unwrap();
	if seen.contains(&a) {
	    continue;
	}
	seen.insert(a);

	if (px, py) == end_pos {
	    result2 = t;
	    break;
	}

	// consider each direction {
	for d in &ds {
	    let nx = px as isize + d.0;
	    let ny = py as isize + d.1;
	    if walls.contains(&(nx , ny)) {

		continue;
	    }

	    // check for blizzards at t +1
	    let mut clear = true;
	    
	    for ((bx, by), b) in blizzards.iter() {
		let (btx, bty) = blizzard_at_step(*bx as isize, *by as isize, *b, t + 1, dim.0, dim.1);

		if btx == nx && bty == ny {
		    clear = false;

		    break;
		}
	    };
	    if !clear {
		continue;
	    }
	    pq.push((nx as isize, ny as isize, t+1), -(t as isize + 1 + (nx - end_pos.0).abs() + (ny - end_pos.1).abs()));
	}
	let mut z = closed_list.entry((px, py)).or_insert(t);
	*z = *z.min(&mut t);
    }
    
    // heuristic: distance to end

    Ok((result1, result2))
}
