// Definisi trait
trait Hewan {
    fn bersuara(&self);
}

// Implementasi trait untuk struct Kucing
struct Kucing;
impl Hewan for Kucing {
    fn bersuara(&self) {
        println!("Meow!");
    }
}

// Implementasi trait untuk struct Anjing
struct Anjing;
impl Hewan for Anjing {
    fn bersuara(&self) {
        println!("Woof!");
    }
}

// Fungsi untuk memanggil metode trait
fn panggil_bersuara<T: Hewan>(hewan: T) {
    hewan.bersuara();
}

fn main() {
    let kucing = Kucing;
    let anjing = Anjing;

    // Memanggil metode trait
    panggil_bersuara(kucing); // Output: Meow!
    panggil_bersuara(anjing); // Output: Woof!
}
