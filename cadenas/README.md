# Cadenas (Strings)

Una cadena de caracteres es una sucesion de caracteres que se almacenan en memoria. En rust, podemos utilizarlos de dos formas.

## Literales

Los literales en Rust, se definen en un texto entre comillas dobles ```"```. Estos se definen como el tipo de dato ```&str```; este tipo de cadenas son inmutables y estaticos; es decir que se almacenan en tiempo de compilación.

```rust
let st="Hola Mundo"; //literal estatico se almacena en tiempo de ejecucion.
```

## Strings

Sin embargo, no siempre sabemos la lontigud o cuando va a estar almacenada nuestra cadena en memoria; por lo que se utilizará el tipo ```std::String```; para poder almacenar estas cadenas mutables.

```rust
let mut cadena:String=String::from("Hola Mundo");//Almacenamos la cadena "Hola mundo como mutable"
```

## Conversion de tipos

Para poder convertir un tipo ```&str``` a ```String```, pueden usarse las siguientes funciones.

```rust
let x:String=String::from("Hola Mundo"); //Convertir de &str a String, creando una nueva cadena.
```

```rust
let x:String= String::from("Hola Mundo");
let x1:&str=&x;//Convertir de String a &str
```

```rust
let x:String= "Hola mundo".to_string();//Convertir de &str a String.
```
