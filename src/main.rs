#[allow(unused)]
use aoc2022::old_days::{day03, day04};
#[allow(unused)]
use aoc2022::*;
#[allow(unused)]
use ndarray::Array2;
#[allow(unused)]
use std::collections::HashMap;
#[allow(unused)]
use std::collections::HashSet;
use std::io::{Result, Write};


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


#[allow(unused)]
fn day_template(use_example: bool) -> Result<(usize, usize)> {
    let day = 7;
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

fn day10(use_example: bool) -> Result<(isize, isize)> {
    let day = 10;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };
    let signals = vec![20, 60 ,100, 140, 180, 220];

    let lines = read_lines(path, true)?;

    // Part 1
    let mut n = 0;
    let mut cycle = 0;
    let mut result1 = 0;
    let mut reg = 1;
    for line in &lines {
	let tok: Vec<_> = line.split(" ").collect();
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
	reg=  new_reg
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
	let tok: Vec<_> = line.split(" ").collect();
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
    for j in 0..6 {
	for i in 0..40 {
	    print!("{}", grid[j][i]);
	}
	print!("\n");
    }

    return Ok((result1, 0));
}
fn main() -> anyhow::Result<()> {
    let day = 10;
    let res_fn = day10;
    let (ex1, ex2) = res_fn(true)?;

    let p1 = format!("{}", ex1);
    let p2 = format!("{}", ex2);
    let _ = check_day(day, p1, p2)?;
    let (part1_ans, part2_ans) = res_fn(false)?;

    let part1_ans_str = format!("{}", part1_ans);
    let part2_ans_str = format!("{}", part2_ans);
    if true {
        println!("{}", part1_ans);
	if ask_part(1)? {
	    println!("Submitting {} to Part 1...", part1_ans_str);
	    submit_answer(2022, day, 1, &part1_ans_str)?;
	}

        println!("{}", part2_ans);
	if ask_part(2)? {
	    println!("Submitting {} to Part 2...", part2_ans_str);
	    submit_answer(2022, day, 2, &part2_ans_str)?;
	}
    }

    Ok(())
}
