# Funciones

En Rust, podemos definir funciones para poder dividir nuestro programa en distintas partes. La sintaxis para crear una función es la siguiente:

```rust
fn name(param1:type1, param2: type2,paramn: typen) -> return_type{
    ...
}
```

Como vemos es obligatorio que todo parámetro tenga un tipo asociado y que tenga además un tipo de vuelta.

## Llamar a una función

Para llamar a una función, simplemente se usa el nombre de esta y los parametros que necesita:

```rust
fn main(){
    ...
    mifuncion(3);
    ...
}
```

## Parámetros

Como hemos podido ver en el ejemplo anterior, a las funciones se les puede pasar parámetros; pudiendose pasar de dos formas:

* Por valor: se crea una copia de la variable y se pasa a la función. Veremos con más detalle este proceso en la gestión de memoria.
* Por referencia: Se pasa una referencia la variable. Veremos con más detalle este proceso en la gestión de memoria.

```rust
fn mifuncion(param1:i32)->i32{ //Paso de parametro por valor
...
```

```rust
fn mifuncion(param1:&i32){//Paso de parametro por referencia
...
```

## Devolver un dato de la función

Para poder devolver un dato, es imprescindible definir de que tipo será en la declaración de esta, usando el operador ```->```.

En Rust, la ultima variable referenciada en una ejecución de una función, se toma como dato de respuesta de la función; ejemplo:

```rust
fn doble(x:i32)->i32{
    2*x //Notese que no hay ; al final
}
```

Sin embargo, tambien puede usarse la instrucción ```return```; Ejemplo:

```rust
fn doble(x:i32)->i32{
    return 2*x;
}
```