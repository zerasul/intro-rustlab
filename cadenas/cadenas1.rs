/* Ejercicio 10: Cadenas en Rust
 * Modificar este ejercicio para que compile
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

fn mifuncion(x:&str){
    println!("{}",x);
}

fn main(){

    let x:String=String::from("Hola Mundo");

    mifuncion(&x);
}