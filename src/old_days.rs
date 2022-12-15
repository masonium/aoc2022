use crate::string::*;
use crate::*;
use anyhow::Result;
use ndarray::Array2;
use std::collections::{HashMap, HashSet};
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
