## Ini adalah repository untuk belajar Rust dan dokumentasinya

---

### Hello World
```
fn main() {
println!("Hello, world!");
}

```
ini adalah cara melakukan hello world di bahasa pemrograman rust.

---
### Cargo (cara melakukan run di rust dengan terminal)
![img.png](img.png)
di rust ada beberapa cara untuk melakukan run yaitu:
1. cargo run, ini akan merun fn main yang utama
2. cargo test "nama unit test" --exact --nocapture, ini akan merun unit test selain fungsi utama.
3. cargo test "nama unit test" --exact, sama dengan sebelumnya bedanya ini hanya melakukan run tanpa mencetak hasilnya

--- 

### Variable

#### Immutable variable
```
#[test]
fn variable(){
    let first_name = "Ghendida";
    let last_name = "Ayari";
    let age = 19;
    println!( "nama depan {} dan nama belakang adalah {} ", first_name, last_name);
    println!( "usianya adalah {} ", age);
}
```
Di rust variable memggunakan kata kunci let. let ini adalah kata kunci untuk variable yang tidak bisa diubah
jadi isi dari variable tetap tidak bisa diubah lagi setelahnya
<br>

#### Mutable variable

```
#[test]
fn variable_mutable(){
    let mut description = "he is handsome";
    println!( " if i should descript him, i will said {} ", description );

    description = "He's ugly ";
    println!( " i hate him because {} ", description );
}

```
