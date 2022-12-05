use ndarray::Array2;
use num_traits::Zero;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

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
        .map(|x: &str| x.to_string().split("\n").map(|x| x.to_string())
	     .filter(|s| !s.is_empty()).collect())
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
