fn main() {
    let a = vec![1, 2, 3];
    let b = a.into_iter();
    let c = b.collect::<Vec<_>>();
    println!("{:?}", c);
}
