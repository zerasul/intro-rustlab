/* Ejercicio 16: Modulos
 * Ejecuta la funcion del modulo mimodulo
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

pub mod mimodulo{
     
    pub fn mifuncion(x:i32)->i32{
        2*x
    }
}

fn main(){
    let x=2;

    println!("El doble de {} es: {}",x, mimodulo::mifuncion(x));

}