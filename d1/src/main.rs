use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let nums = file_to_vec("./input.txt");
    println!("{}", pt2(nums, 2020));
}

fn file_to_vec(fp: &str) -> Vec<i64> {
    let buf = BufReader::new(File::open(fp).unwrap());
    buf.lines()
        .into_iter()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn pt1(nums: Vec<i64>, sumto: i64) -> i64 {
    for i in &nums {
        for j in &nums {
            if i + j == sumto {
                return i * j;
            }
        }
    }
    return -1;
}

fn pt2(nums: Vec<i64>, sumto: i64) -> i64 {
    for i in &nums {
        for j in &nums {
            for k in &nums {
                if i + j + k == sumto {
                    return i * j * k;
                }
            }
        }
    }
    return -1;
}
