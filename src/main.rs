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
    let age: i32 = 19;
    println!( "umurnya adalah : {} ", age);
}
