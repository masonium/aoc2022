use crate::string::*;
use crate::io::*;
use anyhow::Result;

enum Op {
    Mult,
    Add,
}
enum Part {
    Lit(usize),
    Old,
}

struct Operation {
    op: Op,
    parts: [Part; 2],
}

struct Monkey {
    items: Vec<usize>,
    div_test: usize,
    operation: Operation,
    targets: [usize; 2],
}

fn parse_monkey(s: Vec<String>) -> Monkey {
    let items: Vec<usize> = s[1]
	.trim()
	.split(": ")
	.nth(1)
	.unwrap()
	.split(", ")
	.map(as_usize)
	.collect();
    let ops: Vec<_> = s[2].trim().split(' ').collect();
    let op = match ops[4] {
	"*" => Op::Mult,
	"+" => Op::Add,
	_ => panic!(),
    };
    let p0 = match ops[3] {
	"old" => Part::Old,
	_ => Part::Lit(as_usize(ops[3])),
    };

    let p1 = match ops[5] {
	"old" => Part::Old,
	_ => Part::Lit(as_usize(ops[5])),
    };

    let test = as_usize(s[3].split(' ').last().unwrap());
    let t0 = as_usize(s[4].split(' ').last().unwrap());
    let t1 = as_usize(s[5].split(' ').last().unwrap());

    Monkey {
	items,
	div_test: test,
	targets: [t0, t1],
	operation: Operation {
	    op,
	    parts: [p0, p1],
	},
    }
}

pub fn day11(use_example: bool) -> Result<(usize, usize)> {
    let day = 11;
    let path = if use_example {
	format!("input/example{:02}.in", day)
    } else {
	format!("input/{:02}.in", day)
    };

    let groups = read_line_groups(path)?;

    // Part 1
    let mut monkeys: Vec<_> = groups.iter().map(|x| parse_monkey(x.to_vec())).collect();
    let mut counts = vec![0; monkeys.len()];
    for _ in 0..20 {
	for i in 0..monkeys.len() {
	    let mut t = vec![];
	    let mut f = vec![];
	    {
		let m = &mut monkeys[i];
		let mut c = 0;
		while c < m.items.len() {
		    let mut item = m.items[c];
		    let p1 = match m.operation.parts[1] {
			Part::Lit(x) => x,
			_ => item,
		    };
		    let p0 = match m.operation.parts[0] {
			Part::Lit(x) => x,
			_ => item,
		    };
		    item = match m.operation.op {
			Op::Add => {
			    /*println!("{} + {}", p0, p1); */
			    p0 + p1
			}
			Op::Mult => {
			    /*println!("{} * {}", p0, p1);*/
			    p0 * p1
			}
		    };
		    item /= 3;
		    if item % m.div_test == 0 {
			t.push(item);
		    } else {
			f.push(item);
		    }
		    counts[i] += 1;
		    c += 1;
		}
		m.items.clear();
	    }

	    let t0 = monkeys[i].targets[0];
	    let t1 = monkeys[i].targets[1];

	    monkeys[t0].items.extend(&t);
	    monkeys[t1].items.extend(&f);
	}
    }
    counts.sort();
    let result1 = counts[counts.len() - 1] * counts[counts.len() - 2];
    // Part 2
    let mut monkeys: Vec<_> = groups.iter().map(|x| parse_monkey(x.to_vec())).collect();
    let mut counts = vec![0; monkeys.len()];
    let prod: usize = monkeys.iter().map(|x| x.div_test).product();
    for _ in 0..10000 {
	for i in 0..monkeys.len() {
	    let mut t = vec![];
	    let mut f = vec![];
	    {
		let m = &mut monkeys[i];
		let mut c = 0;
		while c < m.items.len() {
		    let mut item = m.items[c];
		    let p1 = match m.operation.parts[1] {
			Part::Lit(x) => x,
			_ => item,
		    };
		    let p0 = match m.operation.parts[0] {
			Part::Lit(x) => x,
			_ => item,
		    };
		    item = match m.operation.op {
			Op::Add => {
			    /*println!("{} + {}", p0, p1); */
			    p0 + p1
			}
			Op::Mult => {
			    /*println!("{} * {}", p0, p1); */
			    p0 * p1
			}
		    };
		    item %= prod;
		    if item % m.div_test == 0 {
			t.push(item);
		    } else {
			f.push(item);
		    }
		    counts[i] += 1;
		    c += 1;
		}
		m.items.clear();
	    }

	    let t0 = monkeys[i].targets[0];
	    let t1 = monkeys[i].targets[1];

	    monkeys[t0].items.extend(&t);
	    monkeys[t1].items.extend(&f);
	}
    }
    counts.sort();
    let result2 = counts[counts.len() - 1] * counts[counts.len() - 2];
    {}

    Ok((result1, result2))
}
