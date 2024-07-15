fn main() {
    let haystack = "sadbutsad";
    let needle = "sad";

    let k = find_first_index(haystack, needle);

    println!("{}", k);
}

fn find_first_index(haystack: &str, needle: &str) -> i32 {
    if haystack.is_empty() || needle.is_empty() || haystack.len() < needle.len() {
        return -1;
    }

    for i in 0..haystack.len() {
        let e = i + needle.len();     

        if e > haystack.len(){
            return -1;
        }

        let x = &haystack[i..e];

        if x == needle {
            return i as i32;
        }
    }

    return -1;
}
