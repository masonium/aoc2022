use ndarray::Array2;
use num_traits::Zero;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;
use scraper::{Html, Selector};
use colored::Colorize;

pub mod old_days;

/// Return the set of lines in the file, optionally removing any empty lines.
pub fn read_lines<P: AsRef<Path>>(p: P, filter_empty: bool) -> std::io::Result<Vec<String>> {
    let b = std::fs::read_to_string(p)?;
    Ok(b.split("\n")
        .map(|x| x.to_string())
        .filter(|x| !filter_empty || !x.is_empty())
        .collect())
}

/// Return the set of lines in the file, optionally removing any empty lines.
pub fn read_line_groups<P: AsRef<Path>>(p: P) -> std::io::Result<Vec<Vec<String>>> {
    let b = std::fs::read_to_string(p)?;
    Ok(b.split("\n\n")
        .map(|x: &str| x
	     .to_string()
	     .split("\n")
             .map(|x| x.to_string())
	     .collect())
        .collect())
}

/// Read a grid of one-digit numbers.
pub fn read_digit_grid<P: AsRef<Path>>(path: P) -> Array2<usize> {
    let lines = read_lines(path, true).unwrap();
    let mut h: Array2<usize> = Array2::zeros((lines.len(), lines[0].len()));

    for (r, l) in lines.iter().enumerate() {
        for (j, c) in l.bytes().enumerate() {
            h[(r, j)] = (c - b'0') as usize;
        }
    }

    h
}

/// Read a grid of space-separated numbers.
pub fn read_number_grid<F: FromStr + Zero + Clone, P: AsRef<Path>>(path: P) -> Array2<F>
where
    <F as FromStr>::Err: Debug,
{
    let lines = read_lines(path, true).unwrap();
    let mut h: Array2<F> = Array2::zeros((lines.len(), lines[0].len()));

    for (r, l) in lines.iter().enumerate() {
        let x: Vec<&str> = l.split(" ").collect();
        for (j, c) in x.iter().enumerate() {
            h[(r, j)] = c.parse::<F>().unwrap();
        }
    }

    h
}

/// Read a grid of characters.
pub fn read_grid<P: AsRef<Path>>(path: P) -> Array2<u8> {
    let lines = read_lines(path, true).unwrap();
    let mut h: Array2<u8> = Array2::zeros((lines.len(), lines[0].len()));

    for (r, l) in lines.iter().enumerate() {
        for (j, c) in l.bytes().enumerate() {
            h[(r, j)] = c;
        }
    }

    h
}

pub fn submit_answer(year: usize, day: usize, part: usize, answer: &str) -> anyhow::Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let session = &read_lines("session.txt", false)?[0];
    let mut request_headers = reqwest::header::HeaderMap::new();
    request_headers.insert(
        reqwest::header::COOKIE,
        reqwest::header::HeaderValue::from_str(&format!("session={}", session))?,
    );

    let part_str = format!("{}", part);
    let form_data = [("level", part_str), 
		     ("answer", answer.to_string())];
    let client = reqwest::blocking::Client::new();
    let body = client.post(url)
        .headers(request_headers)
	.form(&form_data)
	.send()?;

    println!("Status Code: {}", body.status());

    let doc = Html::parse_document(&body.text()?);
    let selector = Selector::parse("main > article").unwrap();
    let article = doc.select(&selector).next().unwrap();

    let article_text = article.text().collect::<Vec<_>>().join(" ");
    
    println!("Content:\n {}", article_text);

    Ok(())
}

pub fn check_day(day: usize, p0: String, p1: String) -> anyhow::Result<(bool, usize)> {
    let answer_path = format!("input/example{:02}_answer.in", day);
    let answers: Vec<String> = read_lines(answer_path, true)?;

    let mut all_match = true;
    all_match = all_match && answers[0] == p0;
    let result_str = if answers[0] == p0 {
	"matches".green().on_black()
    } else {
	"doesn't match".red().on_black()
    };
    println!("Example 1: given answer {} {} calculted {}",
	     format!("{}", answers[0]).bold(),
	     result_str,
	     format!("{}", p0).bold());

    if answers.len() >= 2 {
	all_match = all_match && answers[1] == p1;
	let result_str = if answers[1] == p1 {
	    "matches".green().on_black()
	} else {
	    "doesn't match".red().on_black()
	};
	println!("Example 2: given answer {} {} calculted {}",
		 format!("{}", answers[1]).bold(),
		 result_str,
		 format!("{}", p1).bold());
    }

    Ok((all_match, answers.len()))
}
