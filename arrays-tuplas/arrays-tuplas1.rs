/* Ejercicio 11: Arrays en Rust
 * Mostrar el doble de los primeros 10 numeros y el triple del numero 11 al 20
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

fn main(){
    let a:[i32;20]=[0;20];

    let slice1 = &a[0..10];
    let slice2 = &a[11..20];

    for i in slice1{
        println!("El doble de {} es: {}",i, 2*i);
    }

    for j in slice2{
        println!("El triple de {} es: {}",j,3*j);
    }
}