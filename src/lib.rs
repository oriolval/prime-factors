use std::{u64};

pub fn factors(n: u64) -> Vec<u64> {

    let mut vec: Vec<u64> = Vec::new();
    let mut temp = n;
    let mut i = 2;

    while temp != 1 {
        if temp%i == 0 {
            vec.push(i);
            temp = temp/i;
            i = 2;
        } else{
            i += 1;
        }
    }
    return vec;
}
