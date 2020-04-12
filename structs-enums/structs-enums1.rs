/* Ejercicio 13: Structs en Rust
 * Mostrar la información de un struct que tenga de información el nombre y la edad.
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

struct Persona{
    nombre:String,
    edad:i32
}

impl Persona{
    fn mostrarinfo(self){
       println!("El nombre es: {} y la edad es: {}", self.nombre, self.edad);
    }
}

fn main(){
 let p:Persona=Persona{nombre: String::from("Victor"), edad: 33};

 p.mostrarinfo();
}