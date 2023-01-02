use std::collections::HashMap;
use crate::read_lines;
use anyhow::Result;

fn resolve(s: &str, m: &HashMap<String, String>) -> i128 {
    let x = &m[s];
    if let Ok(a) = x.parse::<i128>() {
	return a;
    };

    let toks: Vec<_> = x.split(" ").collect();
    assert!(toks.len() == 3);
    let a = resolve(toks[0], m);
    let b = resolve(toks[2], m);
    match toks[1] {
	"+" => a + b,
	"-" => a - b,
	"*" => a * b,
	"/" => a / b,
	_ => panic!("no opt")
    }
}

fn resolve_expr(s: &str, m: &HashMap<String, Box<Expr>>) -> Box<Expr> {
    if s == "humn" {
	return Box::new(Expr::Humn);
    }
    let a = &m[s];
    match a.as_ref() {
	&Expr::Lit(_) | &Expr::Humn => a.clone(),
	Expr::Var(a) => resolve_expr(&a, m),
	Expr::Add(a, b) => {
	    if let (Expr::Var(x), Expr::Var(y)) = (a.as_ref(), b.as_ref()) {
		let aa = resolve_expr(x, m);
		let bb = resolve_expr(y, m);
		Box::new(Expr::Add(aa, bb))
	    } else {
		panic!("bad resolution: {:?}, {:?}", &a, &b);
	    }
	},
	Expr::Sub(a, b) => {
	    if let (Expr::Var(x), Expr::Var(y)) = (a.as_ref(), b.as_ref()) {
		let aa = resolve_expr(x, m);
		let bb = resolve_expr(y, m);
		Box::new(Expr::Sub(aa, bb))
	    } else {
		panic!("bad resolution: {:?}, {:?}", &a, &b);
	    }
	},
	Expr::Mult(a, b) => {
	    if let (Expr::Var(x), Expr::Var(y)) = (a.as_ref(), b.as_ref()) {
		let aa = resolve_expr(x, m);
		let bb = resolve_expr(y, m);
		Box::new(Expr::Mult(aa, bb))
	    } else {
		panic!("bad resolution: {:?}, {:?}", &a, &b);
	    }
	},
	Expr::Div(a, b) => {
	    if let (Expr::Var(x), Expr::Var(y)) = (a.as_ref(), b.as_ref()) {
		let aa = resolve_expr(x, m);
		let bb = resolve_expr(y, m);
		Box::new(Expr::Div(aa, bb))
	    } else {
		panic!("bad resolution: {:?}, {:?}", &a, &b);
	    }
	}


    }
}

#[derive(Clone, Debug)]
enum Expr {
    Lit(i128),
    Humn,
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn parse_expr(expr: &str) -> Box<Expr> {
    if expr == "humn" {
	Box::new(Expr::Humn)
    } else if let Ok(a) = expr.parse::<i128>() {
	Box::new(Expr::Lit(a))
    } else {
	let toks: Vec<_> = expr.split(" ").collect();
	let a = Box::new(Expr::Var(toks[0].to_string()));
	let b = Box::new(Expr::Var(toks[2].to_string()));
	Box::new(match toks[1] {
	    "+" => Expr::Add(a, b),
	    "-" => Expr::Sub(a, b),
	    "*" => Expr::Mult(a, b),
	    "/" => Expr::Div(a, b),
	    _ => { panic!("bad parse"); }
	})
    }

}

fn eval(a: &Box<Expr>) -> Box<Expr> {
    Box::new(
	match a.as_ref() {
	    e @ Expr::Lit(a)  => e.clone(),
	    e @ Expr::Humn => e.clone(),
	    Expr::Add(a, b) => {
		let x = eval(a);
		let y = eval(b);
		if let (Expr::Lit(xx), Expr::Lit(yy)) = (x.as_ref(), y.as_ref()) {
		    Expr::Lit(xx + yy)
		} else {
		    Expr::Add(x, y)
		}
	    },
	    Expr::Sub(a, b) => {
		let x = eval(a);
		let y = eval(b);
		if let (Expr::Lit(xx), Expr::Lit(yy)) = (x.as_ref(), y.as_ref()) {
		    Expr::Lit(xx - yy)
		} else {
		    Expr::Sub(x, y)
		}
	    },
	    Expr::Mult(a, b) => {
		let x = eval(a);
		let y = eval(b);
		if let (Expr::Lit(xx), Expr::Lit(yy)) = (x.as_ref(), y.as_ref()) {
		    Expr::Lit(xx * yy)
		} else {
		    Expr::Mult(x, y)
		}
	    },
	    Expr::Div(a, b) => {
		let x = eval(a);
		let y = eval(b);
		if let (Expr::Lit(xx), Expr::Lit(yy)) = (x.as_ref(), y.as_ref()) {
		    Expr::Lit(xx / yy)
		} else {
		    Expr::Div(x, y)
		}
	    },
	    Expr::Var(_) => {
		panic!("not var");
	    }
	})
}


fn solve(x: i128, a: &Expr) -> i128 {
    match a {
	Expr::Humn => x,
	Expr::Add(a, b) => {
	    if let Expr::Lit(aa) = a.as_ref() {
		solve(x - aa, b.as_ref())
	    } else if let Expr::Lit(bb) = b.as_ref() {
		solve(x - bb, a.as_ref())
	    } else {
		panic!("unsolveable")
	    }
	},
	Expr::Sub(a, b) => {
	    if let Expr::Lit(aa) = a.as_ref() {
		solve(aa - x, b.as_ref())
	    } else if let Expr::Lit(bb) = b.as_ref() {
		solve(x + bb, a.as_ref())
	    } else {
		panic!("unsolveable")
	    }
	},
	Expr::Mult(a, b) => {
	    if let Expr::Lit(aa) = a.as_ref() {
		solve(x / aa, b.as_ref())
	    } else if let Expr::Lit(bb) = b.as_ref() {
		solve(x / bb, a.as_ref())
	    } else {
		panic!("unsolveable")
	    }
	},
	Expr::Div(a, b) => {
	    if let Expr::Lit(aa) = a.as_ref() {
		solve(aa / x, b.as_ref())
	    } else if let Expr::Lit(bb) = b.as_ref() {
		solve(x * bb, a.as_ref())
	    } else {
		panic!("unsolveable")
	    }
	},
	_ => {
	    panic!("unsolveable")
	}

    }

}

fn day21(use_example: bool) -> Result<(i128, i128)> {
    let day = 21;
    let path = if use_example {
	format!("input/example{:02}.in", day)
    } else {
	format!("input/{:02}.in", day)
    };

    let lines = read_lines(path, true)?;

    let patterns: HashMap<String, String> = lines.iter().map(|x| {
	let toks: Vec<_> = x.split(": ").collect();
	(toks[0].to_string(), toks[1].to_string())
    }).collect();

    // Part 1
    let result1 = resolve("root", &patterns);

    let mut exprs: HashMap<_, _> = patterns.iter().map(|(a, b)| (a.to_string(), parse_expr(&b))).collect();
    exprs.remove("humn");
    let full_exprs: HashMap<_, _> = exprs.iter().map(|(a, _)| (a, resolve_expr(a, &exprs))).collect();

    let root = &patterns["root"];
    let toks: Vec<_> = root.split(" ").map(|x| x.to_string()).collect();
    let a = &full_exprs[&toks[0]];
    let b = &full_exprs[&toks[2]];

    let a = eval(a);
    let b = eval(b);
    let b = eval(&b);
    let result2 = if let Expr::Lit(bb) = b.as_ref() {
	solve(*bb, a.as_ref())
    } else {
	panic!("not lit");
    };

    Ok((result1, result2))
}
