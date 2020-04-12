# Arrays y Tuplas

En esta seccion vamos a ver como generar colecciones tanto homogeneas (Arrays), como heterogeneas en Rust.

## Arrays

Un Array es una coleccion homogenea de información del mismo tipo, almacenados de forma contigua. En Rust, se deben inicializar todos los componentes del array para poder tener la propiedad del valor correctamente.

```rust
let a:[i32;5]=[0,1,2,3,4];
```

Como vemos en el anterior ejemplo, se debe definir de que tipo será el array y cuantas posiciones se compone este. Los Arrays en Rust comienzan por la posición ```0```.

```rust
let a:[i32;100]=[0..99];//podemos usar rangos para inicializar los array.
```

Podemos usar el bucle ```for```, para recorrer este array:

```rust
let a:[i32;100]=[0..99];
for i in a{
    println!("{}",i);
}
```

Para acceder a una posicion del array se puede usar el operador ```[]```. No olvidar que los incides son en base 0.

```rust
let a:[i32:4]=[0,1,2,3];
println!("{}",a[1]);//1
```

### Slices

Podemos usar un ```slice```, para poder obtener una parte de un array; para ello se usa el operador ```&``` y un rango de posiciones:

```rust
let a:[i32;100]=[0..99];

let suba=&a[0..50];//Obtiene los 50 primeras posiciones de a.
```

## Tuplas

Las tuplas son listas ordenadas heterogeneas; de tal forma que no tienen que tener todos los elementos el mismo tipo.

```rust
let t:(i32,i32,f64)=(2,3,3.124);//Una tupla compuesta por 3 elementos, 2 i32 y 1 f64.
```

Para acceder a las posiciones de la tupla se usa el operador ```.```; seguido de la pisición, empezando por 0.

```rust
let t:(i32,i32,i16)=(1,2,3);
println!("{}"t.0);//1
```

### Deconstruir una tupla

Podemos deconstruir una tupla para poder obtener cada posicion y poder operar con ellas.

```rust
let t:(&str,i32):("Hola",2);
let name,x = t;
println!("name: {}, x: {}",name,x);
```