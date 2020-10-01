use rand::distributions::{Distribution, Uniform}; // 0.6.5
use std::cmp;

fn populate2(num: usize)  -> Vec<u64> {
    let step = Uniform::new(0, 50);
    let mut rng = rand::thread_rng();
    let choices: Vec<_> = step.sample_iter(&mut rng).take(num).collect();
    choices
}

fn get_num_swaps(vec: &[u64]) -> u64 {
    let mut i = 0;
    let n = vec.len();
    let mut j = n-1;
    let mut count_odds = 0;
    let mut count_evens = 0;

    loop {
        let val_i: u64 = vec[i];
        let val_j: u64 = vec[j];
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

struct SwapStats {
    i_odds:usize, 
    i_evens:usize, 
    is_swap:bool, // we need two bools, one for evens and another for odds.
}

use std::fmt;
impl fmt::Display for SwapStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.i_odds, self.i_evens, self.is_swap)
    }
}

fn is_swappable(vec: &[u64]) -> SwapStats {
    let mut i = 0;
    let n = vec.len();
    let mut j = n-1;
    let mut ss: SwapStats = SwapStats { i_odds: 0, i_evens: 0, is_swap: false };

    loop {
        ss.is_swap = false;
        let val_i: u64 = vec[i];
        let val_j: u64 = vec[j];
        println!("{}, {} --> {}, {}", i, j, val_i, val_j);

        // check for odd to swap until we have one.
        if (val_i % 2) == 1 {
            ss.i_odds = i;
            ss.is_swap = true;
            println!("SWAP for odds {}", ss);
        }
        // check for even to swap until we have one.
        if (val_j % 2) == 0 {
            ss.i_evens = j;
            ss.is_swap = true;
            println!("SWAP for events {}", ss);
        }
        println!("SWAP {}", ss); // loop until we have both an odd and even to swap.
        if (ss.is_swap) || (i >= n-1) || (j == 0) || (i == j) {
            println!("break;");
            break;
        }
        i += 1;
        j -= 1;
        println!("");
    }
    ss
}

fn do_the_swaps(vec: Vec<u64>) {
    loop {
        let ss: SwapStats = is_swappable(&vec);
        if ss.is_swap {
            println!("SWAP {}", ss);
            break;
        }
    }
}

fn main() {
    println!("Hello, world!");

    let vec: Vec<u64> = populate2(21);
    println!("{:?}", vec);

    let num:u64 = get_num_swaps(&vec);
    println!("num = {:?}", num);

    do_the_swaps(vec)
}
