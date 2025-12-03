pub fn hello() {
    println!("Hello, Sekai!");
}

pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// pub fn sub(x: i32, y: i32) -> i32 {
//     x - y
// }

pub fn times(x: i32, y: i32) -> i32 {
    x * y
}

pub fn div(x: f32, y: f32) -> f32 {
    x / y
}

// pub fn modu(x: i32, y: i32) -> i32 {
//     x % y
// }

// pub fn pow(x: i32, y: i32) -> i32 {
//     x.pow(y)
// }

// pub fn max(x: i32, y: i32) -> i32 {
//     x.max(y)
// }

// pub fn min(x: i32, y: i32) -> i32 {
//     x.min(y)
// }

pub fn fib(n: u32) -> u32 {
    if n < 2 {
        return n
    } else {
        return fib(n-1) + fib(n-2)
    }
}

pub fn collazt_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else if n % 2 == 1 {
            n = 3 * n + 1;
        }
        len = len + 1;
        dbg!(n);
    }

    return len;
}