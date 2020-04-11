/* Ejercicio 5: Bucles loop
 * Usando un bucle loop calcular los cuadrados de los numeros
 *  hasta que el resultado sea 64.
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

fn main(){

    let mut a:i32= 0;
    loop{

        println!("El cuadrado de {} es {}", a, a*a);
        //TODO: Parar el bucle
        a+=1;
    }
}