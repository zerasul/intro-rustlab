/* Ejercicio 7: declaracion de funciones
 * Declarar una funcion que calcule el doble para un numero par 
 * y el triple para un numero impar.
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

fn doble_triple(x:i32)->i32{
    if x %2 ==0{
        2*x
    }else{
        3*x
    }
}

fn main(){

    println!("El doble de 2 es: {}",doble_triple(2));
    println!("El triple de 3 es: {}", doble_triple(3));
}