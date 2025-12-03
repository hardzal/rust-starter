mod woo;

fn main() {
    woo::hello();

    let x: i32 = 10;
    let y: i32 = 20;
    println!("x: {x}, y: {y}, sum: {}", woo::sum(x, y));

    println!("Kali: {}", woo::times(10, 3));
    println!("Pembagian: {}", woo::div(7.0, 3.0));
    println!("# Fibonacci");
    println!("f(10) -> {}", woo::fib(10));

    println!("# Collaz Length");
    println!("length: {}", woo::collazt_length(11));

    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 0;
    println!("a: {a:?}");

     let t: (i8, bool) = (7, true);
    dbg!(t.0);
    dbg!(t.1);
}
