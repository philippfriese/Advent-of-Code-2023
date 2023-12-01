use crate::utils;

pub fn first(test: bool) {
    let lines = utils::util::load::<u32>(test, file!());
    println!("First: {:?}", lines);
}

pub fn second(test: bool) {
    let lines = utils::util::load::<u32>(test, file!());
    println!("Second: {:?}", lines);
}