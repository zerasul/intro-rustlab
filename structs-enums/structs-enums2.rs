/* Ejercicio 14: Enums en Rust
 * Mostrar la informacion del Enum
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

enum mi_enum{
    Valor1(i32),
    Valor2(i32,i32,i32),
    Valor3{r:i32,g:i32,b:i32}
}

fn main(){
    let n= mi_enum::Valor1(3);

    match n{
        mi_enum::Valor1(n) => println!("Valor1, n es: {}",n),
        mi_enum::Valor2(x,y,z) => println!("Valor2, x: {}, y:{}, z:{}", x,y,z),
        mi_enum::Valor3{r:r,g:g,b:b} => println!("Valor 3, r: {}, g:{}, b: {}",r,g,b)
    }
}