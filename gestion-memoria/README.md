# Gestión de memoria en Rust

Una de las principales características de Rust es su gestión de memoria ya que nos garantiza en tiempo de compilación, que la memoria esta correctamente gestionada gracias a una serie de reglas que hay que tener en cuenta.

Es muy importante conocer estos conceptos ya que serán necesarios para poder crear nuestros programas o librerías de forma segura.

## Ambito (Scope)

En todo momento, una variable estará definido dentro de un ambito ya sea una función, o en un bloque de código. Cuando se sale de este ambito las variables que son definidas en este ambito son eliminadas. Esta comprobación se hace en tiempo de ejecución. Ejemplo:

```rust
fn mifuncion(x:i32)->i32{
    let y:i32=5;
    y*x//Tanto la variable x como la variable y estan definidas en el ambito de la función y serán eliminadas en cuanto termine la función.
}
```


## Propiedad del valor (Ownership)

Uno de los principales conceptos de Rust es la propiedad del valor; es decir, cada valor almacenado en memoria, tiene asignado un propietario (ownership) de forma que este valor solo estará disponible dentro de su ambito.

```rust
let x:Vec<i32>=Vec::new();

let x1=x;

println("x vale {} y x1 vale {}",x,x1);//ERROR, el propietario del valor x ahora es x1; no puede usarse directamente.
```

### Mover o Copiar el valor

Para poder utilizar el valor de las variables sin que nos de error debido a no tener el propietario del valor. Para ello, podemos usar 2 soluciones:

1. Mover el valor

Cuando la variable se trata de un tipo básico, el propio compilador mueve la propiedad de este valor a una nueva variable.

```rust
let x:i32=0;
let x1:i32=x; //El compilador ha movido el valor de una variable a otra.
```

2. Copiar un valor (clonar)

Cuando la variable se trata de un Objeto o tipo no básico, podemos clonar este objeto (si tiene implementada esta opción).

```rust
let x:Vec<i32>= Vec::new();

let x1=x.clone(); //Se clona el objeto para hacer una copia.
```

### Tomar Prestado (Borrowing)

En algunas ocasiones, no es muy eficiente tener que copiar o clonar las variables de forma que en Rust, podemos usar referencias para poder "tomar prestada" la propiedad del valor. Para crear una referencia utilizaremos el operador ```&```.

```rust
let x:Vec<i32>=Vec::new();

let x1=&x; //toma prestado la propiedad de x.
```

Sin embargo, las referencias por defecto son inmutables; por lo que si queremos también poder modificar el valor de este nos dará un error. Es por ello, que se pueden crear referencias mutables.

```rust
let x:Vec<i32>=Vec::new();

let x1=&x;

x1.push(1);//ERROR; Referencia inmutable
```

```rust
let x:Vec<i32>=Vec::new();

let x1=&mut x;

*x1.push(1); // En este caso el operador * se utiliza para referenciar al valor que este referencia (en este caso x).
```

