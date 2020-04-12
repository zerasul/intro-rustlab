/* Ejercicio 8: gestion memoria Rust
 * Modificar este ejercicio para que compile
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

fn anyadir_uno(v:&Vec<i32>,x:i32)->Vec<i32>{
    let mut vec1=v.clone();
    vec1.push(x);
    vec1
}

fn main(){
    let v0 = Vec::new();
    let v1=anyadir_uno(&v0, 3);

    println!("El valor de v0 es: {:?}",v0); //para formatear un vector se usa {:?}

    println!("El valor de v1 es: {:?}",v1);
}