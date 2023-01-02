#[allow(unused)]
fn day23(use_example: bool) -> Result<(usize, usize)> {
    let day = 23;
    let path = if use_example {
	format!("input/example{:02}.in", day)
    } else {
	format!("input/{:02}.in", day)
    };

    let g = read_grid(path);

    let mut grid = {
	let d = g.dim();
	let mut grid = Array2::from_elem((d.0 + 2000, d.1 + 2000), 0);
	for r in 0..d.0 {
	    for c in 0..d.1 {
		if g[(r, c)] == b'#' {
		    grid[(r+1000, c+1000)] = 1
		}
	    }
	}
	grid
    };
    let mut g2 = grid.clone();

    let mut cons = [[(-1, -1), (-1, 0), (-1, 1)],
		    [(1, -1), (1, 0), (1, 1)],
		    [(1, -1), (0, -1), (-1, -1)],
		    [(1, 1), (0, 1), (-1, 1)]];

    let d = grid.dim();
    let mut min_i = d.0;
    let mut max_i = 0;
    let mut min_j = d.1;
    let mut max_j = 0;


    for i in 0..d.0 {
	for j in 0..d.1 {
	    if grid[(i, j)] != 0 {
		min_i = min_i.min(i);
		max_i = max_i.max(i);

		min_j = min_j.min(j);
		max_j = max_j.max(j);
	    }
	}
    }
    let mut start = 0;
    for _ in 0..10 {
	let mut new_loc = HashMap::new();
	let mut num_moves = 0;
	for i in min_i..=max_i {
	    for j in min_j..=max_j {
		// try move
		if grid[(i, j)] != 1 {
		    continue;
		}
		let mut any = false;
		for ii in i-1..=i+1 {
		    for jj in j-1..=j+1 {
			if ii != i || jj != j {
			    if grid[(ii, jj)] != 0 {
				any = true;
				break;
			    }
			}
		    }
		}
		if !any {
		    continue;
		}

		for d in 0..4 {
		    let dirs = cons[(start + d) % 4];
		    if dirs.iter().all(|dir| {
			let ni = i as isize + dir.0;
			let nj = j as isize + dir.1;

			grid[(ni as usize, nj as usize)] == 0

		    }) {
			let dir = dirs[1];
			let new = (i as isize + dir.0, j as isize + dir.1);
			new_loc.entry(new).or_insert_with(|| vec![]).push((i, j));
			num_moves += 1;
			break;
		    }
		}
	    }
	}

	// move to new
	let mut num_overlaps = 0;
	for (new, olds) in new_loc {
	    if olds.len() > 1 {
		num_overlaps += 1;
		continue;
	    }
	    let old = olds[0];
	    assert!(grid[old] == 1);
	    grid[old] = 0;
	    let n = (new.0 as usize, new.1 as usize);
	    assert!(grid[n] == 0);
	    grid[n] = 1;

	    min_i = min_i.min(n.0);
	    max_i = max_i.max(n.0);

	    min_j = min_j.min(n.1);
	    max_j = max_j.max(n.1);
	}
	start += 1;
    }

    let mut min_i = d.0;
    let mut max_i = 0;
    let mut min_j = d.1;
    let mut max_j = 0;


    for i in 0..d.0 {
	for j in 0..d.1 {
	    if grid[(i, j)] != 0 {
		min_i = min_i.min(i);
		max_i = max_i.max(i);

		min_j = min_j.min(j);
		max_j = max_j.max(j);
	    }
	}
    }
    let mut count_empty = 0;
    for i in min_i..=max_i {
	for j in min_j..=max_j {
	    if grid[(i, j)] == 0 {
		count_empty += 1;
	    }
	}
    }

    // Part 1
    let mut result1 = count_empty;
    {}

    let mut min_i = d.0;
    let mut max_i = 0;
    let mut min_j = d.1;
    let mut max_j = 0;


    grid = g2;
    for i in 0..d.0 {
	for j in 0..d.1 {
	    if grid[(i, j)] != 0 {
		min_i = min_i.min(i);
		max_i = max_i.max(i);

		min_j = min_j.min(j);
		max_j = max_j.max(j);
	    }
	}
    }
    for i in min_i..=max_i {
	for j in min_j..=max_j {
	    print!("{}", if grid[(i, j)] == 1 { '#' } else { '.' });
	}
	println!();
    }

    let mut num_moves = 1;
    let mut num_rounds = 0;
    //dbg!(min_i, max_i, min_j, max_j);
    start = 0;
    while num_moves != 0 {
	num_moves = 0;

	// for i in min_i..=max_i {
	//     for j in min_j..=max_j {
	//      print!("{}", if grid[(i, j)] == 1 { '#' } else { '.' });
	//     }
	//     println!();
	// }

	let mut new_loc = HashMap::new();
	for i in min_i..=max_i {
	    for j in min_j ..= max_j {
		// try move
		if grid[(i, j)] != 1 {
		    continue;
		}
		let mut any = false;
		for ii in i-1..=i+1 {
		    for jj in j-1..=j+1 {
			if ii != i || jj != j {
			    if grid[(ii, jj)] != 0 {
				any = true;
				break;
			    }
			}
		    }
		}
		if !any {
		    continue;
		}

		for d in 0..4 {
		    let dirs = cons[(start + d) % 4];
		    if dirs.iter().all(|dir| {
			let ni = i as isize + dir.0;
			let nj = j as isize + dir.1;

			grid[(ni as usize, nj as usize)] == 0

		    }) {
			let dir = dirs[1];
			let new = (i as isize + dir.0, j as isize + dir.1);
			new_loc.entry(new).or_insert_with(|| vec![]).push((i, j));
			break;
		    }
		}
	    }
	}

	// move to new
	for (new, olds) in new_loc {
	    if olds.len() > 1 {
		continue;
	    }
	    num_moves += 1;

	    let old = olds[0];
	    assert!(grid[old] == 1);
	    grid[old] = 0;
	    let n = (new.0 as usize, new.1 as usize);
	    assert!(grid[n] == 0);
	    grid[n] = 1;

	    min_i = min_i.min(n.0);
	    max_i = max_i.max(n.0);

	    min_j = min_j.min(n.1);
	    max_j = max_j.max(n.1);
	}
	println!("------------------");
	//dbg!(min_i, max_i, min_j, max_j);
	start += 1;
	num_rounds += 1;
    }

    // let mut min_i = d.0;
    // let mut max_i = 0;
    // let mut min_j = d.1;
    // let mut max_j = 0;


    // for i in 0..d.0 {
    //  for j in 0..d.1 {
    //      if grid[(i, j)] != 0 {
    //          min_i = min_i.min(i);
    //          max_i = max_i.max(i);

    //          min_j = min_j.min(j);
    //          max_j = max_j.max(j);
    //      }
    //  }
    // }
    for i in min_i..=max_i {
	for j in min_j..=max_j {
	    print!("{}", if grid[(i, j)] == 1 { '#' } else { '.' });
	}
	println!();
    }

    // Part 2
    let mut result2 = num_rounds + 1;
    {}

    Ok((result1, result2))
}
