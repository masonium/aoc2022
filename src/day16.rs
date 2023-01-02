use crate::io::*;
use crate::string::*;
use anyhow::Result;
use ndarray::Array2;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

type Valves = HashMap<usize, (usize, Vec<usize>)>;

fn all_points_shortest(v: &Valves) -> Array2<usize> {
    let mut shortest = Array2::from_elem((v.len(), v.len()), v.len());
    for i in 0..v.len() {
        shortest[(i, i)] = 0;
    }

    for _ in 0..v.len() - 1 {
        let mut new_shortest = shortest.clone();
        for node in v {
            let a = node.0;
            for neighbor in &node.1 .1 {
                for j in 0..v.len() {
                    new_shortest[(*a, j)] = new_shortest[(*a, j)].min(1 + shortest[(*neighbor, j)]);
                }
            }
        }
        shortest = new_shortest;
    }
    shortest
}

pub fn day16(use_example: bool) -> Result<(usize, usize)> {
    let day = 16;
    let path = if use_example {
        format!("input/example{:02}.in", day)
    } else {
        format!("input/{:02}.in", day)
    };

    let lines = read_lines(path, true)?;

    let mut valves: Valves = HashMap::new();
    let mut m: Vec<String> = vec![];

    for line in &lines {
        let toks: Vec<_> = line.split(" ").collect();
        let v = toks[1].to_string();
        if !m.contains(&v) {
            m.push(v.to_string());
        }
        let rate = extract_uints(&line)[0];
        let l = line.replace("valves", "valve").to_string();
        let r = l.split(" valve ").collect::<Vec<_>>()[1];
        let target: Vec<String> = r.split(", ").map(|x| x.to_string()).collect();
        let mut tx = vec![];
        for t in &target {
            let y = t.clone();
            if !m.contains(&y) {
                m.push(y.to_string());
            }
            tx.push(m.iter().position(|x| x == &y).unwrap());
        }

        valves.insert(m.iter().position(|x| x == &v).unwrap(), (rate, tx));
    }

    let mut rem_valves = vec![];
    for v in &valves {
        if v.1 .0 > 0 {
            rem_valves.push(v.0);
        }
    }

    let aps = all_points_shortest(&valves);

    // Part 1

    // A* search on the valves
    // initial
    let mut x = PriorityQueue::new();
    let start = m.iter().position(|x| x == "AA").unwrap();
    let total_rate: isize = rem_valves.iter().map(|x| valves[x].0 as isize).sum();

    x.push(
        (start, 0, rem_valves.len(), 0, 30, total_rate),
        -total_rate * 30,
    );

    let mut max: isize = 0;
    while x.len() > 0 {
        let (v, top) = x.pop().unwrap();

        if max > -top {
            continue;
        }

        // iterator through remainder
        let (curr, open, num, curr_total, rem, rem_rate) = v;
        max = max.max(-curr_total);
        if num == 0 {
            continue;
        }

        for n in &rem_valves {
            let d = aps[(curr, **n)] as isize;
            if rem - d - 1 >= 0 {
                let a: usize = 1 << *n;
                if a & open == 0 {
                    let new_open = open | a;
                    let new_num = num - 1;
                    let new_rem = rem - d - 1;
                    let rate = valves[*n].0 as isize;
                    let new_total = curr_total - new_rem * rate;
                    let new_rem_rate = rem_rate - rate;

                    let new_v = (**n, new_open, new_num, new_total, new_rem, new_rem_rate);
                    let new_h = new_total - new_rem_rate * new_rem;
                    // println!("{:?}, {}", new_v, new_h);
                    x.push(new_v, new_h);
                }
            }
        }
    }
    let result1 = max as usize;

    // Part 2
    let mut max: isize = 0;

    let mut y: PriorityQueue<(usize, isize, usize, isize, usize, isize, isize, isize), isize> =
        PriorityQueue::new();
    y.push(
        (
            start,
            26,
            start,
            26,
            0,
            rem_valves.len() as isize,
            0,
            total_rate,
        ),
        -total_rate * 26,
    );
    let mut skipped = 0;

    while y.len() > 0 {
        let (v, top) = y.pop().unwrap();

        if max > -top {
            skipped += 1;
            continue;
        }

        // iterator through remainder
        let (curr_a, rem_a, curr_b, rem_b, open, num, curr_total, rem_rate) = v;
        if -curr_total > max {
            max = -curr_total;
        }
        if num == 0 {
            continue;
        }

        for n in &rem_valves {
            let d = aps[(curr_a, **n)] as isize;
            if rem_a - d - 1 >= 0 {
                let a: usize = 1 << *n;
                if a & open == 0 {
                    let new_open = open | a;
                    let new_num = num - 1;
                    let new_rem_a = rem_a - d - 1;
                    let rate = valves[*n].0 as isize;
                    let new_total = curr_total - new_rem_a * rate;
                    let new_rem_rate = rem_rate - rate;

                    let new_v = (
                        **n,
                        new_rem_a,
                        curr_b,
                        rem_b,
                        new_open,
                        new_num,
                        new_total,
                        new_rem_rate,
                    );
                    let new_h: isize = new_total - new_rem_rate * new_rem_a;
                    // println!("{:?}, {}", new_v, new_h);
                    if max >= -new_h {
                        skipped += 1;
                        continue;
                    }
                    y.push(new_v, new_h);
                }
            }
            let d = aps[(curr_b, **n)] as isize;
            if rem_b - d - 1 >= 0 {
                let b: usize = 1 << *n;
                if b & open == 0 {
                    let new_open = open | b;
                    let new_num = num - 1;
                    let new_rem_b = rem_b - d - 1;
                    let rate = valves[*n].0 as isize;
                    let new_total = curr_total - new_rem_b * rate;
                    let new_rem_rate = rem_rate - rate;

                    let new_v = (
                        curr_a,
                        rem_a,
                        **n,
                        new_rem_b,
                        new_open,
                        new_num,
                        new_total,
                        new_rem_rate,
                    );
                    let new_h: isize = new_total - new_rem_rate * new_rem_b;
                    // println!("{:?}, {}", new_v, new_h);
                    if max >= -new_h {
                        skipped += 1;
                        continue;
                    }
                    y.push(new_v, new_h);
                }
            }
        }
    }

    println!("skipped: {skipped}");
    let result2 = max as usize;
    // for v in &valves {
    //  if v.1.0 > 0 {
    //      result1 = result1.max( max_path(&valves, &mut cache, 30, 0, rem_valves, num_valves, "".to_string() ));
    //  }
    // }

    // {}

    // Part 2
    {}

    Ok((result1, result2))
}
