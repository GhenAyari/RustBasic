fn main() {
    println!("Hello, world!");

    println!("AMBAAAATUKAAAAAAAMMMM");

    println!("Mas Rusdiiii loh yaaa");
}

#[test]
fn test_bro() {
    println!("Ambaaaa testtttt");
}

#[test]
fn variable(){
    let first_name = "Ghendida";
    let last_name = "Ayari";
    let age = 19;
    println!( "nama depan {} dan nama belakang adalah {} ", first_name, last_name);
    println!( "usianya adalah {} ", age);
}

#[test]
fn variable_mutable(){
    let mut description = "he is handsome";
    println!( " if i should descript him, i will said {} ", description );

    description = "He's ugly ";
    println!( " i hate him because {} ", description );
}

#[test]
fn membuat_shadowing(){
    let nama_tengah = "Gantari";
    println!("nama tengah {}", nama_tengah);

    let nama_tengah = 7;
    println!("nama tengahnya ada {} huruf", nama_tengah);
}

#[test]
fn deklarasi_variable(){
    let age: i8 = 19;
    println!( "umurnya adalah : {} ", age);
    let ipk: f32 = 3.85;
    println!( "ipk adalah : {}", ipk);
    let jarak_ke_kampus: u16 = 20;
    println!("jarak ke kampus adalah : {} {}", jarak_ke_kampus, " Km");

    /* perbedaan u dan i adalah jika u angkanya pasti positif dan tidak mungkin negatif dan jika i angkanya bisa negatif dan bisa positif
    gunakan u jika angkanya atau hasilnya pasti positif dan gunakan i jika ada kemungkinan hasilnya negatif. untuk default dari rustnya sendiri adalah
    i64. jadi misal tidak mendeklarasikan contoh seperti let umur = 19. itu defaultnya i64. semakin besar bitnya semakin boros memori. Dan untuk float
    defaultnya adalah f64
    */

}

#[test]
fn setelah_sekian_lama(){
    println!("Saya kembali belajar rust hehehe");
    let tanggal_kembali: i32 = 21;
    println!( "tanggal kembali ke rust {}", tanggal_kembali);
}

#[test]
fn konversi_tipe_data_number(){
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: u8 = a as u8;
    println!("{}", c);

    /*
        Di rust untuk melakukan konversi tipe data number itu menggunakan kata kunci "as".
     */
}

#[test]
fn latihan_tipe_data_number(){
    let a: u8 = 10;
    let b: i16 = 30;

    // let c: i16 = a as i16; (bisa menambah variable seperti ini jadi nanti let hasil = b + c;
    //     println!("hasil adalah {}", hasil); )

    println!("hasil adalah {}", b + a as i16);

}

#[test]
fn operasi_numerik(){
    /*
    Di rust operasi numerik itu sama dengan bahasa lain. yaitu + untuk tambah, * untuk kali, / untuk bagi, % untuk sisa bagi atau modulus, - untuk kurang
    */

    let a = 10;
    let b = 20;

    let hasil = a + b;
    println!("hasil adalah {}", hasil);

    let c: u8 = 9;
    let d: i16 = 30;

    println!("hasil dari : {} {} ", c as u16 * d as u16, "itulah hasilnya");

}

#[test]
fn augmanted_assignments(){
    let mut a = 5;

    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 3;
    println!("{}", a);

    a *= 10;
    println!("{}", a);

    a %= 3;
    println!("{}", a);

    let mut b = 5;
    println!("{}", b);

    b %= 2;
    println!("{}", b);
}

#[test]
fn boolean(){
    let a = true;
    let b: bool = false;

    println!("ini adalah : {}", a);
    println!("ini adalah salah : {} ", b)
}

#[test]
fn comparison_operators(){
    let hasil: bool = 10 > 30;

    println!("apakah 10 lebih dari 30? {}", hasil);

    let a: u8 = 10;
    let b: i8 = 15;

    let hasil1 = a <= b as u8;

    println!("apakah a lebih dari b? {}", hasil1);
}

#[test]
fn operators_boolean(){
    /*
    di operators boolean ada tiga yaitu && adalah dan, || adalah atau, ! adalah kebalikan
     */

    let absen = 13;
    let nilai = 70;

    let lulus = absen >= absen;
    let lulus_nilai = nilai >= nilai;

    let lulus_final = lulus && lulus_nilai;
    println!("Dia Bagas dinyatakan lulus? {} ", lulus_final);


    let lulus = absen < absen;
    let lulus_nilai = nilai > nilai;

    let lulus_final = lulus && lulus_nilai;
    println!("Dia Andi dinyatakan dinyatakan? {} ", lulus_final);
}

#[test]
fn tipe_char(){
    let a = 'g';
    let b = 'h';
    let c = 'e';
    let d = 'n';

    println!("{}{}{}{}", a, b, c, d);
}


#[test]
fn tuple(){
    /*
     cara penulisan tuple ada banyak contohnya di bawah ini:
     */

    // Cara 1
    let data: (&str, u8, u16) = ("Ghendida", 19, 2006);
    println!("{:?}", data);

    // Cara 2
    let data_baru = ("Ghendida ", 19, 2006);
    println!("Nama {} ", data_baru.0);
    println!("Usia {} ", data_baru.1);
    println!("Tahun lahir {} ", data_baru.2);

    // Cara 3 menggunakan destructing membongkar tuple jadi variabel terpisah
    let (nama, umur, tahun_lahir) = data_baru;
    println!("Nama {} ", nama);
    println!("Umur saat ini {} ", umur);
    println!("Tahun lahir {} ", tahun_lahir);

    // Cara 4 yaitu mutable tuple artinya isinya bisa diubah ubah.

    let mut identitas: (String, i8, i16) = (String::from("Ghendida"), 19, 2006);

    println!("{:?}", identitas);

    identitas.0 = String::from("Ambaaaaa ");
    identitas.1 = 22;
    identitas.2 = 2003;

    println!("{:?}", identitas);

    let (nama, umur, tahun_lahir) = identitas;
    println!("Nama {} ", nama);
    println!("Umur saat ini {} ", umur);
    println!("Tahun lahir {} ", tahun_lahir);

}

fn unit(){
    println!("Ini adalah unit");
}

#[test]
fn test_unit(){
    let hasil = unit();
    println!("{:?}", hasil);

    let test = ();
    println!("{:?}", test);

}

#[test]
fn array(){
    let array1: [i16; 7] = [1, 2, 3, 4, 7, 9, 11];

    println!("{:?}", array1);

    println!("");
    let nama: [&str; 3] = ["Ambacong", "Rusdi", "Fuad"];
    println!("Ketua {} ", nama[1]);
    println!("Member 1 {} ", nama[0]);
    println!("Member 2 {} ", nama[2]);

}