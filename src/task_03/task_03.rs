use std::cmp::min;
use crate::utils;


fn update(valid: &mut [&mut [bool]], i: usize, j: usize) {
    let ia = i+1;
    let ja = j+1;

    valid[ia  ][ja  ] = true;
    valid[ia  ][ja+1] = true;
    valid[ia  ][ja-1] = true;

    valid[ia+1][ja  ] = true;
    valid[ia+1][ja+1] = true;
    valid[ia+1][ja-1] = true;

    valid[ia-1][ja  ] = true;
    valid[ia-1][ja+1] = true;
    valid[ia-1][ja-1] = true;
}
pub fn first(test: bool) {
    let lines = utils::util::load::<String>(test, file!());

    let size_x = lines[0].len();
    let size_y = lines.len();
    println!("{} {}", size_x, size_y);

    let mat = lines.iter().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut temp = vec![false; (size_x+2) * (size_y+2)];
    let mut grid_base: Vec<_> = temp.as_mut_slice().chunks_mut(size_x+2).collect();
    let valid = grid_base.as_mut_slice();

    for i in 0..size_x {
        for j in 0..size_y {
            if !mat[i][j].is_digit(10) && mat[i][j] != '.' {
                update(valid, i,j);
            }
        }
    }

    mat.iter().for_each(|l| println!("{:?}", l));
    valid[1..size_x+1].iter()
        .for_each(|l| println!("{:?}", l[1..size_y+1].iter().map(|b| char::from_digit(*b as u32, 10).unwrap()).collect::<Vec<_>>()));


    let mut numbers = Vec::new();
    for (i,line) in lines.iter().enumerate() {
        let mut num = String::from("");
        let mut is_valid = false;

        for (j,c) in line.chars().enumerate() {
            if c.is_digit(10) {
                num.push(c);
                is_valid |= valid[i+1][j+1];
                if j != line.len()-1 {
                    continue;
                }
            }
            if is_valid && num.len() != 0 {
                numbers.push(num.parse::<u32>().unwrap());
            }
            num = String::from("");
            is_valid = false;

        }

    }
    println!("{:?}", numbers);
    println!("{:?}", numbers.iter().sum::<u32>());

    //println!("First: {:?}", lines);
}


fn get_numbers(mat: &Vec<Vec<char>>, ix: i32, jx:i32) -> Vec<u32> {
    let mut numbers = Vec::new();

    for i in ix-1..=ix+1 {
        let mut num = String::from("");
        let mut is_valid = false;
        for j in jx-3..=jx+3 {
            if !(i >= 0 && i < mat.len() as i32) { continue }
            if !(j >= 0 && j < mat[i as usize].len() as i32) { continue }

            let c = mat[i as usize][j as usize];
            if c.is_digit(10) {
                is_valid |= jx-1 <= j && j <= jx+1;
                num.push(c);
                if j < min(jx+3, mat[i as usize].len() as i32-1) {
                    continue;
                }
            }
            //println!("{:} {:} {:} {} - {}", i,j,c, is_valid, num);
            if num.len() != 0 && is_valid {
                numbers.push(num.parse::<u32>().unwrap());
            }
            num = String::from("");
            is_valid = false;
        }
    }
    numbers
}
pub fn second(test: bool) {
    let lines = utils::util::load::<String>(test, file!());

    let size_x = lines[0].len();
    let size_y = lines.len();
    println!("{} {}", size_x, size_y);

    let mat = lines.iter().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut temp = vec![false; (size_x+2) * (size_y+2)];
    let mut grid_base: Vec<_> = temp.as_mut_slice().chunks_mut(size_x+2).collect();
    let valid = grid_base.as_mut_slice();


    let mut numbers = Vec::new();
    for i in 0..size_x {
        for j in 0..size_y {
            if mat[i][j] == '*' {
                let a = get_numbers(&mat, i as i32,j as i32);
                if a.len() == 2 {
                    numbers.push(a[0] * a[1]);
                }
                println!("{:?}", a);
            }
        }
    }

    println!("{:?}", numbers.iter().sum::<u32>());

    //println!("Second: {:?}", lines);
}