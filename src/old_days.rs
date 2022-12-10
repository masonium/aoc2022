use crate::*;
use std::collections::{HashSet, HashMap};
use crate::string::*;
use std::io;
use anyhow::Result;

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


struct File {
    name: String,
    size: usize
}


struct Dir {
    name: String,
    parent: String,
    files: Vec<File>,
    children: Vec<String>,
}

impl Default for Dir {
    fn default() -> Self {
        Self { name: "".to_string(), parent: "".to_string(), files: vec!(), children: vec![] }
    }
}

impl Dir {
    fn size(&self, dirs: &HashMap<String, Dir>) -> usize {
	let f = self.files.iter().map(|x| x.size).sum::<usize>();
	let c = self.children.iter().map(|x| dirs[x].size(dirs)).sum::<usize>();
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
    let mut root = Dir::default();
    root.name = "/".to_string();
    dirs.insert("/".to_string(), root);

    for line in &lines {
	let toks: Vec<_> = line.split(" ").collect();
	if line.starts_with("$") {
	    if toks[1] == "cd" {
		if toks[2] == ".." {
		    let d = dirs[&curr].parent.clone(); 
		    curr = d;
		} else {
		    curr = format!("{};{}", curr, toks[2].to_string());
		}
	    } else if toks[1] == "ls" {
	    }
	} else {
	    if line.starts_with("dir") {
		let t = toks[1].to_string();
		let subname = format!("{};{}", curr, t);
		dirs.entry(subname.clone()).or_default().parent = curr.clone();
		dirs.entry(subname.clone()).or_default().name = subname.clone();
		dirs.entry(curr.clone()).or_default().children.push(subname.clone());
	    } else {
		let size = as_usize(toks[0]);
		let name = toks[1];
		dirs.entry(curr.clone()).or_default().files.push(File { name: name.to_string(), size });
	    }
	}
    }

    // Part 1
    let result1: usize = dirs.values().map(|x| x.size(&dirs)).filter(|z| *z <= 100000).sum();

    // Part 2
    let size: isize = dirs["/;/"].size(&dirs) as isize;
    println!("{}", size);
    let min_d = size - 40000000;
    println!("{}", min_d);
    let result2 = dirs.values().map(|x| x.size(&dirs))
	.filter(|z| *z >= min_d as usize)
	.min().unwrap();
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
	'outer: for i in 0..dim.0 {
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
	    for x in i+1..dim.0 {
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
	    for y in j+1..dim.1 {
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
    for j in 1..dim.1-1 {
	'outer: for i in 1..dim.0-1 {
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
	    c= 0 ;
	    for x in i+1..dim.0 {
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
	    for y in j+1..dim.1 {
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
	let toks: Vec<&str> = line.split(" ").collect();
	let d = toks[0].bytes().next().unwrap();
	let dir = dirs[&d];
	let num = as_usize(toks[1]);

	for _ in 0..num {
	    head = add(head, dir);

	    if next_to(head, tail) {
		continue;
	    }

	    let mut dx = (head.0 - tail.0);
	    dx = dx.abs().min(1) * dx.signum();
	    let mut dy = (head.1- tail.1);
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
	let toks: Vec<&str> = line.split(" ").collect();
	let d = toks[0].bytes().next().unwrap();
	let dir = dirs[&d];
	let num = as_usize(toks[1]);

	for _ in 0..num {
	    knots[0] = add(knots[0], dir);
	    for i in 1..10 {
		if next_to(knots[i-1], knots[i]) {
		    continue;
		}

		let mut dx = (knots[i-1].0 - knots[i].0);
		dx = dx.abs().min(1) * dx.signum();
		let mut dy = (knots[i-1].1- knots[i].1);
		dy = dy.abs().min(1) * dy.signum();
		knots[i] = (knots[i].0 + dx, knots[i].1 + dy);
	    }
	    positions.insert(knots[9]);
	}

    }

    let result2 = positions.len();
    Ok((result1, result2))
}
