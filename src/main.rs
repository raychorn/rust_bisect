use rand::distributions::{Distribution, Uniform}; // 0.6.5
use std::cmp;

fn populate2(num: usize)  -> Vec<i64> {
    let step = Uniform::new(0, 50);
    let mut rng = rand::thread_rng();
    let choices: Vec<_> = step.sample_iter(&mut rng).take(num).collect();
    choices
}

fn get_num_swaps(vec: Vec<i64>) -> i64 {
    let mut i = 0;
    let n = vec.len();
    let mut j = n-1;
    let mut count_odds = 0;
    let mut count_evens = 0;

    loop {
        let val_i: i64 = vec[i];
        let val_j: i64 = vec[j];
        //println!("{}, {} --> {}, {}", i, j, val_i, val_j);

        if (val_i % 2) == 1 {
            count_odds += 1;
        }
        if (val_j % 2) == 0 {
            count_evens += 1;
        }
        //println!("{} - {}", count_odds, count_evens);
        if (i  >= n-1) || (j == 0) || (i == j) {
            break;
        }
        i += 1;
        j -= 1;
        //println!("");
    }
    //assert_eq!(count_evens, count_odds, "count_evens is {} and count_odds is {} and they are not equal.", count_evens, count_odds);
    cmp::max(count_evens, count_odds)
}

fn main() {
    println!("Hello, world!");

    let vec: Vec<i64> = populate2(21);
    println!("{:?}", vec);

    let num:i64 = get_num_swaps(vec);
    println!("num = {:?}", num);
}
