use std::collections::HashMap;
use crate::utils;

pub fn first(test: bool) {
    let lines = utils::util::load::<String>(test, file!());

    let matches: u32 = lines.iter()
        .map(|line| line
            .chars()
            .filter(|c| c.is_digit(10) ).collect::<Vec<char>>()
            )
        .map(|cs| String::from(cs[0]).to_owned() + &*if cs.len() > 1 { cs.last().unwrap().to_string() } else { String::from(cs[0]) })
        .map(|x| x.parse::<u32>().unwrap())
        .sum();

    println!("{:?}", matches);
}

pub fn replace(line: &str) -> Vec<&str> {
    let mut local_line = line.to_string();
    let lut = vec!
        [
            ("1", "1"),
            ("2", "2"),
            ("3", "3"),
            ("4", "4"),
            ("5", "5"),
            ("6", "6"),
            ("7", "7"),
            ("8", "8"),
            ("9", "9"),
            ("one", "1"),
           ("two", "2"),
           ("three", "3"),
           ("four", "4"),
           ("five", "5"),
           ("six", "6"),
           ("seven", "7"),
           ("eight", "8"),
           ("nine", "9"),
        ];
    let mut hits: Vec<(&str,&str,usize)> = Vec::new();


    for (k,v) in &lut {
        let matches = local_line.match_indices(k).collect::<Vec<_>>();
        if matches.len() > 0 {
            for m in matches {
                hits.push((k,v,m.0));
            }
        }

    }
    hits.sort_by(|(_,_,l),(_,_,r)| l.partial_cmp(r).unwrap());
    hits.iter().map(|(_,v,_)|v.to_owned()).collect::<Vec<_>>()
}

pub fn second(test: bool) {
    let lines = utils::util::load::<String>(test, file!());

    let matches = lines.iter()
        .map(|line| line)
        .map(|line| replace(line)
        )
        .map(|cs| String::from(cs[0]).to_owned() + &*if cs.len() > 1 { cs.last().unwrap().to_string() } else { String::from(cs[0]) })
        .map(|x| x.parse::<u64>().unwrap())
        .sum::<u64>();
    // 54265
    println!("{:?}", matches);
}