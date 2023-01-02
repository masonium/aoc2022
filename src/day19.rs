use std::collections::HashMap;
use crate::{read_lines, extract_uints};
use anyhow::Result;

#[derive(Clone, Debug, Copy)]
struct Plan {
    ore_ore: usize,
    clay_ore: usize,
    obs_ore: usize,
    obs_clay: usize,
    geode_ore: usize,
    geode_obs: usize,
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Robot {
    ore_c: usize,
    clay_c: usize,
    obs_c: usize,
    geode_c: usize,

    ore: usize,
    clay: usize,
    obs: usize,
    geode: usize,
}

type Cache = HashMap<(Robot, usize), usize>;

impl Robot {
    fn new() -> Robot {
	Robot {
	    ore_c: 0,
	    clay_c: 0,
	    obs_c: 0,
	    geode_c: 0,
	    ore: 1,
	    clay: 0,
	    obs: 0,
	    geode: 0,
	}
    }

    fn inc(&mut self, n_inc: usize) {
	self.ore_c += self.ore * n_inc;
	self.clay_c += self.clay * n_inc;
	self.obs_c += self.obs * n_inc;
	self.geode_c += self.geode * n_inc;
    }

    fn target(&self) -> usize{
	self.geode_c
    }
}

impl Plan {
    fn earliest(&self, r: &Robot, steps: usize) -> (usize, usize, usize, usize) {
	// try to build something
	let earliest_ore = if self.ore_ore <= r.ore_c {
	    0
	} else {
	    (self.ore_ore - r.ore_c + r.ore - 1) / r.ore
	};

	let earliest_clay = if self.clay_ore <= r.ore_c {
	    0
	} else {
	    (self.clay_ore - r.ore_c + r.ore - 1) / r.ore
	};

	let earliest_obs = {
	    let mut d = 0;
	    if self.obs_ore > r.ore_c {
		if r.ore > 0 {
		    d = d.max( (self.obs_ore - r.ore_c + r.ore - 1) /  r.ore)
		} else {
		    d = steps + 1;
		}
	    }
	    if self.obs_clay > r.clay_c {
		if r.clay > 0 {
		    d = d.max( (self.obs_clay - r.clay_c + r.clay - 1) /  r.clay)
		} else {
		    d = steps + 1;
		}
	    }
	    d
	};

	let earliest_geode = {
	    let mut d = 0;
	    if self.geode_ore > r.ore_c {
		if r.ore > 0 {
		    d = d.max( (self.geode_ore - r.ore_c + r.ore - 1) /  r.ore)
		} else {
		    d = steps + 1;
		}
	    }
	    if self.geode_obs > r.obs_c {
		if r.obs > 0 {
		    d = d.max( (self.geode_obs - r.obs_c + r.obs - 1) /  r.obs)
		} else {
		    d = steps + 1;
		}
	    }
	    d
	};
	(earliest_ore, earliest_clay, earliest_obs, earliest_geode)
    }


    fn best_aux(&self, steps: usize, r: &Robot, c: &mut Cache) -> usize {
	let mut rr = r.clone();
	rr.geode_c = 0;
	let key = (rr.clone(), steps);
	if !c.contains_key(&key) {
	    let v = self.best2(steps, &&rr, c);
	    c.insert(key.clone(), v);
	}
	c[&key] + r.geode_c

    }


    fn best2(&self, steps: usize, r: &Robot, c: &mut Cache) -> usize {
	//println!("{:?}", r);
	if steps == 0 {
	    return r.target();
	}

	let (earliest_ore, earliest_clay, earliest_obs, earliest_geode) = self.earliest(r, steps);


	// collect the ore
	let rr = {
	    let mut rr = r.clone();
	    rr.inc(1);
	    rr
	};

	let mut best_max = rr.geode_c;

	if earliest_ore < steps {
	    let mut ro = rr.clone();
	    ro.inc(earliest_ore);
	    ro.ore_c -= self.ore_ore;
	    ro.ore += 1;
	    let bm = self.best_aux(steps - earliest_ore - 1, &ro, c);
	    if bm > best_max  {
		best_max = bm;
	    }
	}
	if earliest_clay < steps {
	    let mut ro = rr.clone();
	    ro.inc(earliest_clay);
	    ro.ore_c -= self.clay_ore;
	    ro.clay += 1;
	    let bm = self.best_aux(steps - earliest_clay - 1, &ro, c);
	    if bm > best_max  {
		best_max = bm;
	    }
	}
	if earliest_obs < steps {
	    let mut ro = rr.clone();
//	    println!("steps = {}, before: {:?}", steps, &ro);
	    ro.inc(earliest_obs);
	    // println!("after: {:?}", &ro);
	    // dbg!(&ro, self.obs_ore, self.obs_clay);
	    ro.ore_c -= self.obs_ore;
	    ro.clay_c -= self.obs_clay;
	    ro.obs += 1;
	    let bm = self.best_aux(steps - earliest_obs - 1, &ro, c);
	    if bm > best_max  {
		best_max = bm;
	    }
	}
	if earliest_geode < steps {
	    let mut ro = rr.clone();
	    //println!("need {earliest_geode} steps to get {} ore and {} obsidian.", ro.ore_c, ro.obs_c);
	    ro.inc(earliest_geode);
	    ro.ore_c -= self.geode_ore;
	    ro.obs_c -= self.geode_obs;
	    ro.geode += 1;
	    let bm = self.best_aux(steps - earliest_geode - 1, &ro, c);
	    if bm > best_max  {
		best_max = bm;
	    }
	}

	// don't build anything again;
	{
	    let mut ro = rr.clone();
	    ro.inc(steps-1);

	    if ro.target() > best_max  {
		best_max = ro.target();
	    }
	}

	best_max
    }

}



#[allow(unused)]
fn day19(use_example: bool, nd: usize) -> Result<(usize, usize)> {
    let day = 19;
    let path = if use_example {
	format!("input/example{:02}.in", day)
    } else {
	format!("input/{:02}.in", day)
    };

    let plans: Vec<_> = read_lines(path, true)?
	.iter()
	.map(|x| {
	    let nums = extract_uints(&x);
	    println!("{:?}", nums);
	    Plan {
		ore_ore: nums[1],
		clay_ore: nums[2],
		obs_ore: nums[3],
		obs_clay: nums[4],
		geode_ore: nums[5],
		geode_obs: nums[6],
	    }
	})
	.map(|x| {
	    println!("{:?}", x);
	    x
	})
	.collect();

    // Part 1
    let mut result1 = plans
	.iter()
	.enumerate()
	.map(|(i, x)| {
	    let mut c = Cache::new();
	    let bm = x.best2(nd, &Robot::new(), &mut c);
	    (i+1) * bm
	})
	.sum();
    {}

    // Part 2
    let mut result2 = plans.iter()
	.take(3)
	.map(|x| {
	    let mut c = Cache::new();
	    x.best2(32, &Robot::new(), &mut c)
	})
	.fold(1, |a, b| a*b);

    Ok((result1, result2))
}
