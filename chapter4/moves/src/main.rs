fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s; // value moved here
    //let u = s;  // value used here after move

    // 明示的にコピーがほしいことを要求する .clone();
    let s1 = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t1 = s1.clone();
    let u1 = s1.clone();
}
