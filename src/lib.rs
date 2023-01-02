use colored::Colorize;
use scraper::{Html, Selector};

pub mod day11;
pub mod day13;
pub mod day16;
pub mod io;
pub mod old_days;
pub mod string;
pub mod day19;
pub mod day21;
pub mod day22;

pub use io::*;
pub use string::*;

pub fn submit_answer(year: usize, day: usize, part: usize, answer: &str) -> anyhow::Result<bool> {
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let session = &read_lines("session.txt", false)?[0];
    let mut request_headers = reqwest::header::HeaderMap::new();
    request_headers.insert(
        reqwest::header::COOKIE,
        reqwest::header::HeaderValue::from_str(&format!("session={}", session))?,
    );

    let part_str = format!("{}", part);
    let form_data = [("level", part_str), ("answer", answer.to_string())];
    let client = reqwest::blocking::Client::new();
    let body = client
        .post(url)
        .headers(request_headers)
        .form(&form_data)
        .send()?;

    println!("Status Code: {}", body.status());

    let doc = Html::parse_document(&body.text()?);
    let selector = Selector::parse("main > article").unwrap();
    let article = doc.select(&selector).next().unwrap();

    let article_text = article.text().collect::<Vec<_>>().join(" ");

    println!("Content:\n {}", article_text);

    Ok(article_text.contains("That's the right answer")
        || article_text.contains("already complete it"))
}

pub fn check_day(day: usize, p0: &str, p1: &str) -> anyhow::Result<(bool, usize)> {
    let answer_path = format!("input/example{:02}_answer.in", day);
    let answers: Vec<String> = read_lines(answer_path, true)?;

    let mut all_match = true;
    all_match = all_match && answers[0] == p0;
    let result_str = if answers[0] == p0 {
        "matches".green().on_black()
    } else {
        "doesn't match".red().on_black()
    };
    println!(
        "Example 1: given answer {} {} calculted {}",
        answers[0].bold(),
        result_str,
        p0.bold()
    );

    if answers.len() >= 2 {
        all_match = all_match && answers[1] == p1;
        let result_str = if answers[1] == p1 {
            "matches".green().on_black()
        } else {
            "doesn't match".red().on_black()
        };
        println!(
            "Example 2: given answer {} {} calculted {}",
            answers[1].bold(),
            result_str,
            p1.bold()
        );
    }

    Ok((all_match, answers.len()))
}

pub fn o_neighbors2(x: usize, y: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nx = x as isize + d.0;
        let ny = y as isize + d.1;
        if nx >= 0 && ny >= 0 && nx < n as isize && ny < m as isize {
            res.push((nx as usize, ny as usize))
        }
    }
    res
}

pub fn d_neighbors2(x: usize, y: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for d in [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, 1),
        (-1, -1),
        (1, -1),
        (1, 1),
    ] {
        let nx = x as isize + d.0;
        let ny = y as isize + d.1;
        if nx >= 0 && ny >= 0 && nx < n as isize && ny < m as isize {
            res.push((nx as usize, ny as usize))
        }
    }
    res
}

/// Return all orthogonal neighbors of a point in a 3d lattices.
pub fn o_neighbors3(
    p: (usize, usize, usize),
    s: (usize, usize, usize),
) -> Vec<(usize, usize, usize)> {
    let mut res = vec![];
    for d in [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ] {
        let nx = p.0 as isize + d.0;
        let ny = p.1 as isize + d.1;
        let nz = p.2 as isize + d.2;
        if nx >= 0
            && ny >= 0
            && nz >= 0
            && nx < s.0 as isize
            && ny < s.1 as isize
            && nz < s.2 as isize
        {
            res.push((nx as usize, ny as usize, nz as usize))
        }
    }
    res
}

/// Return all neighbors (orthogonal and diagonal) of  a point in a 3d lattices.
pub fn d_neighbors3(
    p: (usize, usize, usize),
    s: (usize, usize, usize),
) -> Vec<(usize, usize, usize)> {
    let mut res = vec![];
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            for dz in [-1, 0, 1] {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                let nx = p.0 as isize + dx;
                let ny = p.1 as isize + dy;
                let nz = p.2 as isize + dz;
                if nx >= 0
                    && ny >= 0
                    && nz >= 0
                    && nx < s.0 as isize
                    && ny < s.1 as isize
                    && nz < s.2 as isize
                {
                    res.push((nx as usize, ny as usize, nz as usize))
                }
            }
        }
    }
    res
}
