#[allow(unused)]
use aoc2022::old_days::{day03, day04};
#[allow(unused)]
use aoc2022::*;
#[allow(unused)]
use ndarray::Array2;
#[allow(unused)]
use std::collections::HashSet;
use std::io::{Result, Write};

#[allow(unused)]
fn day_template(use_example: bool) -> Result<(usize, usize)> {
    let day = 4;
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

fn day06(use_example: bool) -> Result<(usize, usize)> {
    let day = 6;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let _lines = read_lines(path, false)?;

    // Part 1
    let mut result1 = 0;

    // Part 2
    let mut result2 = 0;

    Ok((result1, result2))
}

fn ask_part(part: usize) -> anyhow::Result<bool> {
    let mut stdout = std::io::stdout();
    print!("Would you like to submit part {}? [y/n] ", part);
    stdout.flush()?;
    let stdin = std::io::stdin();
    let mut ans: String = "".to_string(); 
    stdin.read_line(&mut ans)?;
    println!("{}", ans);
    
    Ok(ans.starts_with("y"))
}

fn main() -> anyhow::Result<()> {
    let day = 4;
    let res_fn = day04;
    let (ex1, ex2) = res_fn(true)?;

    let p1 = format!("{}", ex1);
    let p2 = format!("{}", ex2);
    let (check, num_ans) = check_day(day, p1, p2)?;
    let (part1_ans, part2_ans) = res_fn(false)?;

    let part1_ans_str = format!("{}", part1_ans);
    let part2_ans_str = format!("{}", part2_ans);
    if check {
        println!("{}", part1_ans);
        if num_ans == 1 {
	    if ask_part(1)? {
		println!("Submitting {} to Part 1...", part1_ans_str);
		submit_answer(2022, day, 1, &part1_ans_str)?;
	    }
        }

        println!("{}", part2_ans);
        if num_ans == 2 {
	    if ask_part(2)? {
		println!("Submitting {} to Part 2...", part2_ans_str);
		submit_answer(2022, day, 2, &part2_ans_str)?;
	    }
        }
    }

    Ok(())
}
