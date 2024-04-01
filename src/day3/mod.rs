use std::{
    fs::File,
    io::{BufRead, BufReader},
    mem,
    path::Path,
};

// Day 3
pub fn run() {
    let file_path = Path::new("./src/day3/input");
    println!("{}", file_path.display());

    let fp = File::open(file_path).expect("Unable to open file");
    let fp = BufReader::new(fp);

    let lines: Vec<String> = fp.lines().flatten().collect();
    let mut nums: Vec<String> = Vec::new();

    for i in 0..lines.len() {
        let mut num = String::new();
        let l: Vec<char> = lines[i].chars().collect();
        let mut j = 0;
        while j < l.len() {
            let cur_ch = l[j];
            if cur_ch.is_digit(10) {
                if check_adj_sym(i, j, &lines) {
                    j = read_all_num(&mut num, &lines[i], j);
                    nums.push(mem::take(&mut num));
                } else {
                    num.push(cur_ch);
                }
            } else {
                num = String::new();
            }
            j += 1;
        }
    }
    println!(
        "{:?}",
        nums.iter()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
            .iter()
            .sum::<u64>()
    );
}

fn check_adj_sym(cur_i: usize, cur_j: usize, lines: &Vec<String>) -> bool {
    let max_i = lines.len();

    let prv_i = cur_i.checked_sub(1);
    if prv_i.is_some_and(|x| x < max_i) {
        if any_adj(&lines[prv_i.unwrap()], cur_j) {
            return true;
        }
    }

    let nxt_i = cur_i.checked_add(1);
    if nxt_i.is_some_and(|x| x < max_i) {
        if any_adj(&lines[nxt_i.unwrap()], cur_j) {
            return true;
        }
    }
    return any_adj(&lines[cur_i], cur_j);
}

fn any_adj(l: &String, cur_j: usize) -> bool {
    let chars: Vec<char> = l.chars().collect();
    let max_j = chars.len();

    // println!("{:?}", chars);

    let prv_j = cur_j.checked_sub(1);
    if prv_j.is_some_and(|x| x < max_j) {
        if is_valid_sym(&chars, prv_j.unwrap()) {
            return true;
        }
    }

    let nxt_j = cur_j.checked_add(1);
    if nxt_j.is_some_and(|x| x < max_j) {
        if is_valid_sym(&chars, nxt_j.unwrap()) {
            return true;
        }
    }

    return is_valid_sym(&chars, cur_j);
}

fn is_valid_sym(chars: &Vec<char>, idx: usize) -> bool {
    if chars[idx] != '.' && !chars[idx].is_digit(10) {
        return true;
    }
    return false;
}

fn read_all_num(num: &mut String, l: &String, idx: usize) -> usize {
    let chars: Vec<char> = l.chars().collect();
    for i in idx..chars.len() {
        if chars[i].is_digit(10) {
            (*num).push(chars[i])
        } else {
            return i;
        }
    }
    return idx;
}
