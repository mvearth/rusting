fn main() {
    let mut nums = vec![0, 0, 0, 1, 2, 3];
    let val = 0;

    let k = remove(&mut nums, val);

    println!("Removed {} of {} founded in the array.", k, val);
}

fn remove(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut k = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }

    return k as i32;
}
