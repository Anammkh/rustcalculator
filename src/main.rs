use std::io;

fn main() {
    println!("Pilih operasi:");
    println!("1. Penjumlahan");
    println!("2. Pengurangan");
    println!("3. Perkalian");
    println!("4. Pembagian");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Gagal membaca input");

    let choice: u32 = choice.trim().parse().expect("Masukkan angka yang valid");

    println!("Masukkan angka pertama:");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Gagal membaca input");
    let num1: f64 = num1.trim().parse().expect("Masukkan angka yang valid");

    println!("Masukkan angka kedua:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Gagal membaca input");
    let num2: f64 = num2.trim().parse().expect("Masukkan angka yang valid");

    match choice {
        1 => println!("{} + {} = {}", num1, num2, num1 + num2),
        2 => println!("{} - {} = {}", num1, num2, num1 - num2),
        3 => println!("{} * {} = {}", num1, num2, num1 * num2),
        4 => {
            if num2 == 0.0 {
                println!("Tidak bisa membagi dengan nol!");
            } else {
                println!("{} / {} = {}", num1, num2, num1 / num2);
            }
        }
        _ => println!("Pilihan tidak valid!"),
    }
}

