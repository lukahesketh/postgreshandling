fn main() {
    let x: i32 = 24;
    let y = &x;

    assert_eq!(x, *y);
}
