use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("===Game tebak angka===");
    println!("Tebak angka dari 1 sampai 100");

    let angka_rahasia = rand::thread_rng().gen_range(1..=100);

    let mut coba = 0;

    loop {
        if coba >= 5 {
            println!("Maaf, kesempatan menebak kamu sudah habis!");
            println!("Angka rahasia adalah: {}", angka_rahasia);
            break;
        }

        println!("Tebak angka: ");

        let mut tebakan = String::new();

        io::stdin()
            .read_line(&mut tebakan)
            .expect("Failed to read line");

       let tebakan: u32 = match tebakan.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
       };
        coba += 1;
        
       println!("Kamu menebak: {}", tebakan);

       match tebakan.cmp(&angka_rahasia) {
            Ordering::Less => println!("Terlalu ekcil!"),
            Ordering::Greater => println!("Terlalu besar!"),
            Ordering::Equal => {
                println!("Selamat, Tebakan kamu benar!");
                break;
            }
       }
    }
    println!("Kamu menebak {} kali", coba);
}