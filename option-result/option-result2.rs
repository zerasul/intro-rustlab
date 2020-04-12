/* Ejercicio 16: Result
 * Escribir el codigo que falta
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

fn mifuncion(x:i32)->Result<i32,&'static str>{
   
    if x>0 {
        Ok(x*x*x)
    }else{
        Err("Número negativo")
    }
}

fn main(){

    let x=5;

    match mifuncion(x) {
        Ok(n) => println!("El cubo de {} es: {}", x,n),
        Err(message) => println!("Ha ocurrido un error: {}",message)
    }
}