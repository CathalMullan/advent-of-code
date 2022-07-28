use std::error::Error;
use std::str::FromStr;

pub fn by_line_vector<T: FromStr>(file_contents: &str) -> Vec<T>
where
    <T as FromStr>::Err: Error,
{
    file_contents
        .split('\n')
        .map(|line| line.trim().parse().unwrap())
        .collect()
}
