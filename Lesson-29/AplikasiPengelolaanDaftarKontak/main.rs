use std::collections::{HashMap, HashSet, VecDeque};

// Struktur data untuk menyimpan informasi kontak
#[derive(Clone)] // Derive the Clone trait for Contact
struct Contact {
    name: String,
    phone: String,
}

fn main() {
    // Membuat Vec untuk daftar kontak
    let mut contacts: Vec<Contact> = Vec::new();

    // Membuat HashMap untuk mencari kontak berdasarkan nama
    let mut contact_map: HashMap<String, Contact> = HashMap::new();

    // Membuat HashSet untuk menyimpan kategori kontak
    let mut contact_categories: HashSet<String> = HashSet::new();

    // Membuat VecDeque untuk antrian pesan dari kontak
    let mut message_queue: VecDeque<String> = VecDeque::new();

    loop {
        println!("Menu:");
        println!("1. Tambah Kontak");
        println!("2. Cari Kontak");
        println!("3. Tampilkan Semua Kontak");
        println!("4. Kirim Pesan");
        println!("5. Keluar");
        
        let choice = input_number("Pilih menu: ");

        match choice {
            1 => {
                let name = input_string("Nama kontak: ");
                let phone = input_string("Nomor telepon: ");
                let contact = Contact { name: name.clone(), phone };
                
                contacts.push(contact.clone()); // Menambahkan kontak ke dalam Vec
                contact_map.insert(name.clone(), contact.clone()); // Menambahkan kontak ke dalam HashMap
                
                let category = input_string("Kategori kontak: ");
                contact_categories.insert(category);
                
                println!("Kontak berhasil ditambahkan!");
            }
            2 => {
                let name = input_string("Nama kontak yang dicari: ");
                if let Some(contact) = contact_map.get(&name) {
                    println!("Kontak ditemukan:");
                    println!("Nama: {}", contact.name);
                    println!("Nomor Telepon: {}", contact.phone);
                } else {
                    println!("Kontak tidak ditemukan.");
                }
            }
            3 => {
                println!("Daftar Kontak:");
                for contact in &contacts {
                    println!("Nama: {}", contact.name);
                    println!("Nomor Telepon: {}", contact.phone);
                    println!("---");
                }
            }
            4 => {
                if let Some(message) = message_queue.pop_front() {
                    println!("Mengirim pesan: {}", message);
                } else {
                    println!("Tidak ada pesan dalam antrian.");
                }
            }
            5 => {
                println!("Terima kasih telah menggunakan aplikasi!");
                break;
            }
            _ => {
                println!("Pilihan tidak valid. Silakan pilih menu yang benar.");
            }
        }
    }
}

// Fungsi untuk menerima input angka dari pengguna
fn input_number(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Gagal membaca input.");
        
        match input.trim().parse::<i32>() {
            Ok(number) => return number,
            Err(_) => println!("Input tidak valid. Masukkan angka yang benar."),
        }
    }
}

// Fungsi untuk menerima input string dari pengguna
fn input_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Gagal membaca input.");
    input.trim().to_string()
}
