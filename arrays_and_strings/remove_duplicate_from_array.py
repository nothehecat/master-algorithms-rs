#!/usr/bin/env run-cargo-script

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut previous = None;
    let mut j = 0;

    for i in 0..nums.len() {
        if Some(nums[i]) != previous {
            previous = Some(nums[i]);
            nums.swap(i, j);
            j += 1;
        }
    }

    j as i32
}
