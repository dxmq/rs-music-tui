#[test]
fn test() {
    let vec1 = [1, 2, 3, 4, 5, 6];
    let offset = 1;
    let limit = 5;
    let mut add = offset + limit;
    if add > vec1.len() {
        add = vec1.len();
    }
    let a = &vec1[offset..(add)];
    println!("{:?}", a);
}
