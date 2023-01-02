use std::collections::HashMap;
use crate::read_line_groups;
use ndarray::Array2;
use anyhow::Result;
use colored::Colorize;

enum Inst {
    Left,
    Right,
    F(usize)
}

fn day22(use_example: bool) -> Result<(usize, usize)> {
    let day = 22;
    let path = if use_example {
	format!("input/example{:02}.in", day)
    } else {
	format!("input/{:02}.in", day)
    };

    let groups = read_line_groups(path)?;

    let max_col = groups[0].iter().map(|x| x.len()).max().unwrap();
    let mut m = ndarray::Array2::from_elem((groups[0].len(), max_col), ' ');

    for (i, line) in groups[0].iter().enumerate() {
	for (j, c) in groups[0][i].chars().enumerate() {
	    m[(i, j)] = c;
	}
    }
    let password = &groups[1][0];
    let mut s: usize = 0;
    let mut insts = vec![];
    for c in password.chars() {
	if '0' <= c && c <= '9' {
	    s = s * 10 + (c as u8 - b'0') as usize;
	} else if c == 'R' {
	    if s > 0 {
		insts.push(Inst::F(s));
		s =  0;
		insts.push(Inst::Right);
	    }
	} else if c == 'L' {
	    if s > 0 {
		insts.push(Inst::F(s));
		s =  0;
		insts.push(Inst::Left);
	    }
	}
    }
    if s > 0 {
	insts.push(Inst::F(s));
	s = 0;
    }
    let mut pos: (isize, isize) = (0, 0);
    let d = m.dim();
    let di = (d.0 as isize, d.1 as isize);
    for j in 0..d.1 {
	if m[(0,j)] == '.' {
	    pos.1 = j as isize;
	    break;
	}
    }
    let start_pos = pos;
    println!("start_pos: {:?}", start_pos);

    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dir = 0;
    for i in &insts {
	assert!(m[(pos.0 as usize, pos.1 as usize)] == '.');
	match i {
	    Inst::Left => {
		dir = (dir + 3) % 4;
	    },
	    Inst::Right => {
		dir = (dir + 1) % 4;
	    },
	    Inst::F(steps) => {
		for _ in 0..*steps {
		    let dd = DIRS[dir];
		    let mut next = (pos.0 + dd.0, pos.1 + dd.1);
		    if dd.0 > 0 {
			if next.0 >= di.0 {
			    next.0 = 0;
			}
			while m[(next.0 as usize, next.1 as usize)] == ' ' {
			    next.0 = (next.0 + 1) % di.0;
			}
		    }
		    if dd.0 < 0 {
			if next.0 < 0 {
			    next.0 = di.0 - 1;
			}
			while m[(next.0 as usize, next.1 as usize)] == ' ' {
			    next.0 = (next.0 + di.0 - 1) % di.0;
			}
		    }
		    if dd.1 > 0 {
			if next.1 >= di.1 {
			    next.1 = 0;
			}
			while m[(next.0 as usize, next.1 as usize)] == ' ' {
			    next.1 = (next.1 + 1) % di.1;
			}
		    }
		    if dd.1 < 0 {
			if next.1 < 0 {
			    next.1 = di.1 - 1;
			}
			while m[(next.0 as usize, next.1 as usize)] == ' ' {
			    next.1 = (next.1 + di.1 - 1) % di.1;
			}
		    }
		    if m[(next.0 as usize, next.1 as usize)] == '#' {
			break;
		    }

		    pos = next;
		}
	    }
	}
    }


    // Part 1
    let mut result1 = (1000 * (pos.0 + 1) + 4 * (pos.1 + 1)) as usize + dir;

    if use_example {
	return Ok((result1, 0));
    }

    let mut next: HashMap< (isize, isize, usize), (isize, isize, usize)> = HashMap::new();
    for i in 0..d.0 {
	for j in 0..d.1 {
	    if m[(i, j)] == ' ' {
		continue;
	    }
	    for d in 0..4 {
		let dd = DIRS[d];
		let mut n = (i as isize + dd.0, j as isize + dd.1);
		if n.0 < 0 {
		    n.0 = di.0 - 1;
		}
		if n.0 >= di.0 {
		    n.0 = 0;
		}
		if n.1 < 0 {
		    n.1 = di.1 - 1;
		}
		if n.1 >= di.1 {
		    n.1 = 0;
		}

		next.insert((i as isize, j as isize, d),
			    (n.0, n.1, d));
	    }
	}
    }

    for w in 0..50 {
	// C
	next.insert((49, 100+w, 1), (50+w, 99, 2));
	next.insert((50+w, 99, 0), (49, 100+w, 3));
	assert!(m[(49 as usize, (100+w) as usize)] != ' ');
	assert!(m[(50+w as usize, 99 as usize)] != ' ');

	// A
	next.insert((w, 149, 0), (149-w, 99, 2));
	next.insert((149-w, 99, 0), (w, 149, 2));
	assert!(m[(w as usize, (149) as usize)] != ' ');
	assert!(m[(149-w as usize, 99 as usize)] != ' ');


	// B
	next.insert((149, 50+w, 1), (150+w, 49, 2));
	next.insert((150+w, 49, 0), (149, 50+w, 3));
	assert!(m[(149 as usize, (50+w) as usize)] != ' ');
	assert!(m[((150+w) as usize, 49 as usize)] != ' ');

	// D
	next.insert((w, 50, 2), (149-w, 0, 0));
	next.insert((149-w, 0, 2), (w, 50, 0));
	assert!(m[(w as usize, 50 as usize)] != ' ');
	assert!(m[(149-w as usize, 0 as usize)] != ' ');

	// F
	next.insert((0, 50 + w, 3), (150 + w, 0, 0));
	next.insert((150 + w, 0, 2), (0, 50 + w, 1));
	assert!(m[(0 as usize, (50+w) as usize)] != ' ');
	assert!(m[((150+w) as usize, 0 as usize)] != ' ');

	// G
	next.insert((100, w, 3), (50 + w, 50, 0));
	next.insert((50 + w, 50, 2), (100, w, 1));
	assert!(m[(100 as usize, (w) as usize)] != ' ');
	assert!(m[((50+w) as usize, 50 as usize)] != ' ');

	// E
	next.insert((0, 100 + w, 3), (199, w, 3));
	next.insert((199, w, 1), (0, 100 + w, 1));
	assert!(m[(0 as usize, (100+w) as usize)] != ' ');
	assert!(m[((199) as usize, w as usize)] != ' ');
    }

    for (a, b) in next.iter() {
	println!("{:?}", a);
	println!("{:?}", b);
	assert!(m[(b.0 as usize, b.1 as usize)] != ' ');
    }

    let mut m2 = m.clone();
    #[allow(unused)]
    fn print_m2(m2: &Array2<char>, pd: &(isize, isize, usize)) {
	let d = m2.dim();
	println!("------------");
	for i in 0.. d.0 {
	    for j in 0..d.1 {
		if pd.0 as usize == i && pd.1 as usize == j {
		    print!("{}", format!("{}", m2[(i, j)]).on_white().black());
		} else {
		    print!("{}", m2[(i, j)])
		}
	    }
	    println!("");
	}
	let mut stdin = std::io::stdin();
	use std::io::Read;
	let _ = stdin.read(&mut [0u8]);
    }


    let dc = ['>', 'v', '<', '^'];

    let mut pd = (start_pos.0, start_pos.1, 0);
    m2[(start_pos.0 as usize, start_pos.1 as usize)] = dc[0];
    for i in &insts {
	assert!(m[(pd.0 as usize, pd.1 as usize)] == '.');
	match i {
	    Inst::Left => {
		pd.2 = (pd.2 + 3) % 4;
		m2[(pd.0 as usize, pd.1 as usize)] = dc[pd.2 as usize];
//		print_m2(&m2, &pd);
//		println!("{}", pd.2);
	    },
	    Inst::Right => {
		pd.2 = (pd.2 + 1) % 4;
		m2[(pd.0 as usize, pd.1 as usize)] = dc[pd.2 as usize];
//		print_m2(&m2, &pd);
//		println!("{}", pd.2);
	    },
	    Inst::F(steps) => {
		for _ in 0..*steps {
		    let next_pd = next[&pd];
//		    println!("From: {:?}", pd);
//		    println!("To:   {:?}", next_pd);
		    let a = m[(next_pd.0 as usize, next_pd.1 as usize)];
		    assert!(a != ' ');
		    if a == '#' {
			break;
		    }

		    pd = next_pd;
		    m2[(pd.0 as usize, pd.1 as usize)] = dc[pd.2 as usize];
//		    print_m2(&m2, &pd);
		}
	    }
	}
    }


    // Part 2
    let result2 = ((pd.0+ 1) * 1000 + (pd.1 + 1) * 4 + pd.2 as isize) as usize;

    Ok((result1, result2))
}
