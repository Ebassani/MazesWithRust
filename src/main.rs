fn main() {
    let mut vec: Vec<(u32,u32)> = Vec::new();
    vec.push((1,1));
    vec.push((1,2));
    println!("{}",vec.pop().unwrap().1);
    println!("{}",vec.pop().unwrap().1);
}
