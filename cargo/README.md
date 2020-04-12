# Cargo

Cargo es el gestor de dependencias y construccion que utiliza Rust; normalmente se instala al instalar Rust, y permite generar tanto nuestros programas, como crear librerías que podemos publicar en el repositorio de paquetes de Rust e incluso descargar dependencias de este.

Hasta ahora, hemos estado utilizando el compilador de Rust, para poder compilar nuestros programnas; esta práctica no es recomendable ya que siempre es mejor realizando a través de cargo; ya que entre otras cosas, nos ayudará a organizar nuestro codigo e incluso a ejecutar tests.

Por ejemplo para crear un nuevo proyecto con cargo, utilizaremos el siguiente comando:

```bash
cargo init miproyecto
```

Esto generará un proyecto nuevo, y creara una carpeta llamada miproyecto; que contiene los siguientes elementos:

* Carpeta SRC: Carpeta con los fuentes de nuestro proyecto.
* Cargo.toml: Es el manifiesto de nuestro proyecto y es donde definiremos los datos de este e incluso las dependencias que tiene.

Veamos un Ejemplo:

```
[package]
name = "miproyecto"
version = "0.1.0"
authors = ["zerasul <zerasul@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

Como vemos hay 2 apartados; el primero con la etiqueta ```[package]``` contiene la información de nuestro proyecto; y la segunda, contiene las dependencias que tendrá nuestro proyecto.

Una vez que tenemos creado nuestro proyecto, podemos compilarlo con la opción ```build```; ejemplo:

```bash
cargo build
```

El cual nos devuelve lo siguiente:

```bash
Compiling miproyecto v0.1.0 (E:\development\introrust\cargo\miproyecto)
    Finished dev [unoptimized + debuginfo] target(s) in 0.90s
```

En este caso al no especificar nada realiza una compilación sin optimizar; Si queremos optimizar usaremos la opción ```--release```; obteniendo el binario optimizado.

**NOTA:** En la carpeta target, se encontrará los binarios compilados.

Para más información acerca de ```cargo```, puede ir a la página oficial de [Rust](https://doc.rust-lang.org/cargo/).

Este es el final del laboratorio de Rust; Si quieres saber más, puedes ir a la página oficial de Rust, o aprender con ejercicios usando el tutorial [Rustlings](https://github.com/fmoko/rustlings).