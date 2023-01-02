#[allow(unused)]
use aoc2022::old_days::{day03, day04, day24};
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
use priority_queue::PriorityQueue;

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
type P2 = (isize, isize);
#[allow(unused)]
const DIRS: [P2; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];


fn main() -> anyhow::Result<()> {
    let day = 24;
    let res_fn = day24;
    let _args: Vec<_> = std::env::args().collect();

    let (ex1, ex2) = res_fn(true)?;

    let p1 = format!("{}", ex1);
    let p2 = format!("{}", ex2);
    let _ = check_day(day, &p1, &p2)?;

    let (part1_ans, part2_ans) = res_fn(false)?;

    let part1_ans_str = format!("{}", part1_ans);
    let part2_ans_str = format!("{}", part2_ans);
    if true {
        check_day(day, &p1, &p2)?;
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
