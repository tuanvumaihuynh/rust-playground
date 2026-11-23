fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a = [1, 2, 3];
    let a: [i32; 3] = [1, 2, 3];

    let first = a[0];
    let second = a[1];
}
