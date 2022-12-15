#[allow(unused)]
use aoc2022::old_days::{day03, day04};
#[allow(unused)]
use aoc2022::*;
#[allow(unused)]
use ndarray::Array2;
#[allow(unused)]
use std::cmp::Ordering;
#[allow(unused)]
use std::collections::HashMap;
#[allow(unused)]
use std::collections::HashSet;
use std::io::{Result, Write};

fn mark_submit_correct(day: usize, part: usize) -> anyhow::Result<()> {
    let iname = format!("input/.submit_{}_{}", day, part);
    let p = std::path::Path::new(&iname);
    std::fs::File::options().create(true).write(true).open(p)?;
    Ok(())
}

fn ask_part(day: usize, part: usize) -> anyhow::Result<bool> {
    let iname = format!("input/.submit_{}_{}", day, part);
    if std::path::Path::new(&iname).exists() {
        println!("Already submitted day {} part {}.", day, part);
        return Ok(false);
    }

    let mut stdout = std::io::stdout();
    print!("Would you like to submit part {}? [y/n] ", part);
    stdout.flush()?;
    let stdin = std::io::stdin();
    let mut ans: String = "".to_string();
    stdin.read_line(&mut ans)?;
    println!("{}", ans);

    Ok(ans.starts_with('y') || ans.starts_with('Y'))
}

#[allow(unused)]
fn day_template(use_example: bool) -> Result<(usize, usize)> {
    let day = 12;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let _lines = read_lines(path, false)?;

    // Part 1
    let mut result1 = 0;
    {}

    // Part 2
    let mut result2 = 0;
    {}

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

fn main() -> anyhow::Result<()> {
    let day = 15;
    let res_fn = day15;
    let (ex1, ex2) = res_fn(true, 10, 20)?;

    let p1 = format!("{}", ex1);
    let p2 = format!("{}", ex2);
    let _ = check_day(day, p1, p2)?;
    let (part1_ans, part2_ans) = res_fn(false, 2000000, 4000000)?;

    let part1_ans_str = format!("{}", part1_ans);
    let part2_ans_str = format!("{}", part2_ans);
    if true {
        println!("Part 1: {}", part1_ans);
        println!("Part 2: {}", part2_ans);

        if ask_part(day, 1)? {
            println!("Submitting {} to Part 1...", part1_ans_str);
            if submit_answer(2022, day, 1, &part1_ans_str)? {
                mark_submit_correct(day, 1)?;
            }
        }

        if ask_part(day, 2)? {
            println!("Submitting {} to Part 2...", part2_ans_str);
            if submit_answer(2022, day, 2, &part2_ans_str)? {
                mark_submit_correct(day, 2)?;
            }
        }
    }

    Ok(())
}
