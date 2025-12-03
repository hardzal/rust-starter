fn main() {
    // println!("Hello, world!");

    let first = "Hello"; // immmutable
    println!("Welcome, {}!", first);

    // first = "Isekai"; // secara default variabel di rust imuttable
    println!("Welcome to the, {}!", first);

    let mut second = "Isekai e";
    println!("Welcome to the, {}!", second);
    second = "Isekai";
    println!("Welcome to the, {}!", second);

    // array 
    println!("# Immutable Array");
    let arr:[i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {arr:?}");
    println!("{}, {}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3], arr[4]);

    println!("# Mutable Array");
    let mut arr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {arr1:?}");
    println!("{}, {}, {}, {}, {}", arr1[0], arr1[1], arr1[2], arr1[3], arr1[4]);
    arr1[2] = 0;
    println!("Array: {arr1:?}");
    println!("{}, {}, {}, {}, {}", arr1[0], arr1[1], arr1[2], arr1[3], arr1[4]);

    let arr_size: usize = arr1.len();
    println!("panjang arr1: {}", arr_size);

    let arr_two: [[i32; 2]; 2] =[
        [1,2],
        [3,4],
    ];

    println!("{:?}", arr_two);
    println!("{}, {}", arr_two[0][0], arr_two[0][1]);
    println!("{}, {}", arr_two[1][0], arr_two[1][1]);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);
}

#[test]
fn hello_test() {
    println!("Hello, world!");
}

#[test]
fn hello_fail_test() {
    println!("Hello, isekai!");
}