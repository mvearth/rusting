fn main() {
    let mut nums = vec![0, 0, 1, 2, 2, 3];
    let k = rm_duplicates(&mut nums);
    println!("Array length after removing duplicates: {}", k);
    println!("Array after removing duplicates: {:?}", &nums[..k as usize]);
}

fn rm_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut k = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[k - 1] {
            nums[k] = nums[i];
            k += 1;
        }
    }

    return k as i32;
}
