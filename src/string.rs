use std::fmt::Debug;
use std::str::FromStr;

/// Infallibly parse a string as usize
pub fn as_usize(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}
pub fn as_isize(s: &str) -> isize {
    s.parse::<isize>().unwrap()
}
pub fn as_f64(s: &str) -> f64 {
    s.parse::<f64>().unwrap()
}

pub fn as_usize_radix(s: &str, radix: u32) -> usize {
    usize::from_str_radix(s, radix).unwrap()
}

pub fn as_isize_radix(s: &str, radix: u32) -> isize {
    isize::from_str_radix(s, radix).unwrap()
}

/// Parse a string as list of convertible items (usually numbers)
/// separated by delim.
pub fn num_list<T: FromStr>(s: &str, delim: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    s.split(delim).map(|x| x.parse::<T>().unwrap()).collect()
}

/// Parse a string as list of convertible items (usually numbers)
/// separated by delim and ignoring non-numbers
pub fn num_list_ignore<T: FromStr>(s: &str, delim: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    s.split(delim).filter_map(|x| x.parse::<T>().ok()).collect()
}
