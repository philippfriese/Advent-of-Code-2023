use std::cmp::{max, Ordering};
use std::string::ParseError;
use crate::utils;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32
}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lt(other) { Some(Ordering::Less) }
        else if self.gt(other) { Some(Ordering::Greater) }
        else if self.eq(&other) { Some(Ordering::Equal) }
        else { None }
    }
    fn lt(&self, other: &Self) -> bool { self.blue < other.blue && self.red < other.red && self.green < other.green }
    fn le(&self, other: &Self) -> bool { self.blue <= other.blue && self.red <= other.red && self.green <= other.green }
    fn gt(&self, other: &Self) -> bool { self.blue > other.blue && self.red > other.red && self.green > other.green }
    fn ge(&self, other: &Self) -> bool { self.blue >= other.blue && self.red >= other.red && self.green >= other.green }
}

impl std::str::FromStr for Set {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        s.trim().split(",").for_each(|part| {
            let parts = part.trim().split(" ").collect::<Vec<_>>();
            if parts[1] == "red" { set.red = parts[0].parse().unwrap(); }
            else if parts[1] == "green" { set.green = parts[0].parse().unwrap(); }
            else if parts[1] == "blue" { set.blue = parts[0].parse().unwrap(); }
        });
        Ok(set)
    }
}

impl Set {
    fn merge(&self, other: &Set) -> Set {
        Set {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub sets: Vec<Set>
}

impl std::str::FromStr for Game {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(":").collect::<Vec<_>>();
        let id = parts[0].split(" ").skip(1).next().unwrap().parse().unwrap();
        let sets = parts[1].split(";").map(|part| part.parse::<Set>().unwrap()).collect();
        Ok(Game { id, sets } )
    }
}

pub fn first(test: bool) {
    let lines = utils::util::load::<Game>(test, file!());
    let constraint = Set { red: 12, green: 13, blue: 14 };
    let a = lines.iter().filter(|game| {
        let mut r = true;
        game.sets.iter().for_each(|set| r &= (*set <= constraint));
        r
    }).fold(0, |acc, game| acc + game.id);

    println!("First: {:?}", a);
}

pub fn second(test: bool) {
    let lines = utils::util::load::<Game>(test, file!());
    let constraint = Set { red: 0, green: 0, blue: 0 };
    let a = lines.iter().map(|g| g.sets.iter().fold(constraint, |acc, set| acc.merge(set)))
        .map(|g| g.power())
        .sum::<u32>();

    println!("Second: {:?}", a);
}