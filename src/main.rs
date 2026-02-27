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
