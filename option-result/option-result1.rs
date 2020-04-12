/* Ejercicio 15: Option
 * Crear una funcion que devuelva el cuadrado de un numero positivo; en caso de establecer un negativo devolver None.
 * Autor: Victor Suarez <zerasul@gmail.com>
*/
//TODO: definir y escribir la funcion

//fn mifuncion....


fn main(){

    let x:i32=-2;

    match mifuncion(x) {
        Some(n) => println!("El cuadrado de {} es: {}", x,n),
        None => println!("No se permite llamar a la función con numeros negativos")
    }
}