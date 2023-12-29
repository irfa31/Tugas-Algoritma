use std::io;

// Definisikan struktur data untuk menyimpan informasi buku
#[derive(Debug)]
struct Buku {
    judul: String,
    penulis: String,
    tahun_terbit: u32,
}

fn main() {
    // Buat vektor untuk menyimpan buku-buku di perpustakaan
    let mut daftar_buku: Vec<Buku> = Vec::new();

    loop {
        println!("Pilih operasi:");
        println!("1. Tambah buku");
        println!("2. Lihat daftar buku");
        println!("3. Keluar");

        let mut opsi = String::new();
        io::stdin().read_line(&mut opsi).expect("Gagal membaca baris");

        let opsi: u32 = match opsi.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input tidak valid. Masukkan nomor yang valid.");
                continue;
            }
        };

        match opsi {
            1 => tambah_buku(&mut daftar_buku),
            2 => lihat_daftar_buku(&daftar_buku),
            3 => {
                println!("Keluar dari program.");
                break;
            }
            _ => {
                println!("Opsi tidak valid. Masukkan nomor yang valid.");
            }
        }
    }
}

fn tambah_buku(daftar_buku: &mut Vec<Buku>) {
    println!("Masukkan judul buku:");
    let mut judul = String::new();
    io::stdin().read_line(&mut judul).expect("Gagal membaca baris");

    let judul = judul.trim().to_string();

    println!("Masukkan nama penulis:");
    let mut penulis = String::new();
    io::stdin().read_line(&mut penulis).expect("Gagal membaca baris");

    let penulis = penulis.trim().to_string();

    println!("Masukkan tahun terbit:");
    let mut tahun_terbit = String::new();
    io::stdin().read_line(&mut tahun_terbit).expect("Gagal membaca baris");

    let tahun_terbit: u32 = match tahun_terbit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input tidak valid. Masukkan angka yang valid.");
            return;
        }
    };

    let buku = Buku {
        judul,
        penulis,
        tahun_terbit,
    };
    daftar_buku.push(buku);

    println!("Buku berhasil ditambahkan!");
}

fn lihat_daftar_buku(daftar_buku: &Vec<Buku>) {
    println!("Daftar Buku:");
    for buku in daftar_buku {
        println!("{:?}", buku);
    }
}
