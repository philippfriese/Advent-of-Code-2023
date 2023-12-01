use std::fs;
use std::path::Path;
pub fn load_data(fname: &str) -> String {
    fs::read_to_string(fname).expect("Oops")
}

pub fn load<T>(test: bool, this: &str) -> Vec<T>
    where <T as std::str::FromStr>::Err: std::fmt::Debug, T: std::str::FromStr {
    let basepath = Path::new(this).parent().unwrap();

    return if test {
        split_data::<T>(load_data(basepath.join("test.dat").to_str().unwrap()))
    } else {
        split_data::<T>(load_data(basepath.join("data.dat").to_str().unwrap()))
    }
}

pub fn split_data<T>(data: String) -> Vec<T>
    where <T as std::str::FromStr>::Err: std::fmt::Debug, T: std::str::FromStr{
    return data
        .split("\n")
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>();
}