/* Ejercicio 9: gestion memoria Rust
 * Modificar este ejercicio para que compile
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

fn anyadir_uno(v:&Vec<i32>,x:i32)->Vec<i32>{

    let mut v=v.clone();
    v.push(x);
    v
}

fn main(){
    let v0 = Vec::new();
    let v1=anyadir_uno(&v0, 3);

    println!("El valor de v0 es: {:?}",v0); //para formatear un vector se usa {:?}
    v1.push(4);
    println!("El valor de v1 es: {:?}",v1);
}