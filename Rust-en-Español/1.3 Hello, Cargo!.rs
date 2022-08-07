     
    /  /  / Bienvenidos a mis notas sobre Rust. /  /  /

Aca van a encontrar todo lo que me parece importante tener en cuenta
a la hora de convertirse en un Rustacean.

Toda la info existente en este file es un derivado directo 
de "The book", nombre clave de la comunidad para el manual de Rust.

Recuerden que los $ son para hacer referencia a lo que debe ser escrito en 
la consola. A la hora de utilizar el comando se debe omitir.
Los espero del otro lado!.

Voy a saltearme la instalacion, ya que eso va por cuenta de cada uno.

        /  /  / MISSING FOREWORD AND INTRODUCTION /  /  /

        /  /  / MISSING 1.1 AND 1.2 /  /  /

Unit 1.3 : Hello, Cargo!
Cargo es el sistema de "construido" (por falta de una mejor palabra) de Rust y
el manager de las paquetes(packages) (llamados "cajas" en Rust).

Cargo no solo es util, es necesario. Se encarga de descargar las dependencias  
que tu codigo necesita para correr correctamente. Este ya viene instalado por default con Rust. Si bien para un programa simple no necesita mas que las depencencias basicas, a medida que el projecto se vuelve mas grande, Cargo se encargara de que descargar dichas dependencias sea lo mas rapido y sencillo posible.
Si queres confirmar que tenes Cargo instalado es tan facil como hacer:

    $cargo --version

        /  /  / Crear un Projecto con Cargo  /  /  /

Directamente desde la consola (yo ocupo GitBash) podes crear un projecto con:

$cargo new (nombre)
$cargo new hello_cargo <- por ejemplo
$cd hello_cargo <- para entrar a la carpeta del projecto.

Al crear el projecto, si entras al archivo de hello_cargo te daras cuenta que
se generaron dos archivos y un directory: Un archivo "Cargo.toml" y 
un directorio llamado "src", dentro del ultimo encontraras el file "main.rs" donde debera ir todo el codigo.

Tambien se creo en este momento un repositorio en git, junto con un archivo 
"gitignore". No se generaran archivos Git si usas "cargo new" dentro de un
repositorio existente. Pero esto se puede cambiar este comportamiento si se utiliza el comando "cargo new --vcs=git" al crearlo. 

Aparte de crear el projecto, podes buildearlo, que es algo diferente.
No tiene nada que ver con escribir codigo en si, sino mas con compilar
el codigo ya existente. Se debe estar dentro de la carpeta del archivo que creaste con "$cargo new" previamente para invocar el build:
Estando dentro de ~Desktop/Projectos/Rust/hello_cargo (desde la consola)

    $cargo build 

El anterior comando se encarga de compilar el codigo y de crear un acceso directo, ejecutable, de nuestro archivo, al que podremos invocar desde la consola, o bien clickeandolo desde la carpeta en si. 
Para invocar el archivo desde la consola, podés ocupar el siguiente comando:

    $./target/debug/hello_cargo

Genuinamente no se porque es que lleva debug en el comando, supongo que
esa duda se me ira eventualmente.
Si abris Cargo.toml veras algo como esto.

    [package]
    name = "hello_cargo"
    version = "0.1.0"
    edition = "2021"

    [dependencies]

El lenguaje toml(Tom's Obvious, Minimal Language) es el lenguaje que se utiliza
    para las configuraciones de Cargo.
La primera linea [package] es un heading que nos indica que las siguientes 
declaraciones estan configurando un paquete. A medida que sumamos informacion a este archivo, se le sumaran otras secciones.

Las siguientes tres lineas determinan las configuraciones que cargo necesita 
para correr el archivo, el nombre del archivo, la version y la edicion de Rust a usar.

La ultima linea, las [dependencies] es el comienzo de donde estara la lista de 
dependencias que esten instaladas en tu projeto. En Rust, nos referimos a los packages como Crates, los llamaremos de esta manera de ahora en mas.

No necesitaremos ninguna Crate por el momento, pero si utilizaremos alguna en la unidad 2, asi que volveremos a esta seccion de [dependencies] en su momento.
Ahora abramos src/main.rs y veamos que sucede.

Filename: src/main.rs

fn main() {
    println("Hello, Cargo!");
}

Cargo acaba de generar un programa "Hello, Cargo", como el que hicimos en el 
1-1!.Hasta ahora, la diferencia entre el projecto anterior es que al generar Cargo el projecto, situa todo el codigo dentro de el directorio "src". Y que tenemos un archivo Cargo.toml para la configuracion en el directorio principal del projecto.

Cargo siempre esperará que tu codigo este dentro de los archivos "src", ya que
el directorio principal esta destinado unicamente para los archivos README, informacion sobre las licencias, archivos de configuracion, y todo lo demas que no tiene relacion con tu codigo. Usar Cargo te permite organizar tu projecto. En Cargo todo tiene un lugar, y todo está en su lugar.

Si creaste un projecto que no utilice Cargo (Como el "Hello, World"), podes
transformarlo en un projecto que si utilice Cargo. Move el codigo del projecto al directorio "src" y creá un Cargo.toml apropiado.

        /  /  / Armar y Correr un projecto con Cargo  /  /  /

Ahora vamos a ver que es diferente cuando construimos un projecto y corremos
el programa "Hello, world" con Cargo! 
Desde tu directorio <i>Hello_cargo</i>, arma tu projecto con el siguiente
comando:

$cargo build
Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
 Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs

Este comando crea un ejecutable en <i>target/debug/hello_cargo</i>(o en
target\debug\hello_cargo.exe en Windows) y no en tu directorio actual.
Podes correr el ejecutable con el siguiente comando: 

$ ./target/debug/hello_cargo  (o .\target\debug\hello_cargo.exe en Windows)
Hello, world!

Si todo sale bien, "Hello, world!" deberia aparecer en la terminal. Correr
"cargo build" por primera vez tambien causa que Cargo cree un archivo en el nivel superior: Cargo.lock. Este archivo lleva registro de las versiones de las dependencias de tu projecto. Este projecto no tiene muchas dependencias, por lo que este archivo esta un poco vacio. Nunca vas a tener que cambiar este archivo; Cargo maneja sus contenidos por vos.

Acabamos de contruir un projecto con "cargo build" y lo corrimos con 
./target/debug/hello_cargo, pero tambien podriamos usar "cargo run" para compilar el codigo y luego correr el ejecutable resultante, todo un un solo comando.

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!

Noten que esta vez no recibimos informacion acerca de como Cargo estaba 
compilando el hello_cargo. Cargo se dio cuenta que los archivos no cambiaron, asi que simplemente corrio el binario. Si se hubieran modificando los archivos del "src", Cargo habria reconstruido el projecto
antes de correrlo, y habrias visto la siguiente informacion.

$ cargo run
    Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
     Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
      Running `target/debug/hello_cargo`
 Hello, world!

Cargo tambien posee un comando llamado "cargo check". Este comando rapidamente 
se asegura que tu codigo compile pero sin generar un ejecutable.

Porque no querrias un ejecutable? A menudo, "cargo check" es mas rapido que 
"cargo build", simplemente porque se saltea el paso de generar un ejecutable.

Si estas constantemente chequeando tu codigo, usar "cargo check" acelera mucho el proceso! De esa manera, muchos Rustaceans usan "cargo check" periodicamente a medida que escriben codigo para asegurarse de que compile. Luego corren "cargo build" cuando estan listos para correr el ejectuable.

    Hagamos un repaso de lo que aprendimos hasta ahora sobre Cargo:
-Podemos crear un projecto usando "cargo new"
-Podemos armar un projecto usando "cargo build"
-Podemos armar y correr un projecto en un solo paso usando "cargo run"
-Podemos armar un projecto sin producir un ejecutable con "cargo check"
-En vez de guardar el resultado de la "build" en el mismo directorio que
nuestro codigo, Cargo lo guarda en el directorio target/debug.

        /  /  / Armando para una edicion final  /  /  /

Cuando tu projecto esta finalmente listo para realizar una edicion final, podes
usar "cargo build --release" compilar con optimizaciones. Este comando crea un ejecutable en <i>target/release</i> en vez de <i>target/debug</i>.

Las optimizaciones hacen que tu codigo de Rust corra mas rapido, pero hacer realentiza el compilado. Es por esto que hay dos perfiles diferentes: uno para el desarrollo del codigo, cuando queres hacer cambios rapida y constantemente, y otro para el programa final que le daras a tus usuarios,  el cual cambiara repetidas veces y correrá lo mas rapido posible. Si estas haciendo tomando notas de el tiempo que tarde tu codigo en correr, asegurate de usar "cargo build --release" y corre el ejecutable en target/release.

            /  /  / Cargo como la norma /  /  /

Con projectos simples, Cargo no tiene mas utilidad sobre usar "rustc", pero 
probará su valor a medida que tus programas se vuelven mas intricados. Con projectos complejos, compuestos de multiples crates, es mucho mas facil permetirle a Cargo coordinar su construccion por ti.

A pesar de que el projecto hello_cargo es simple, utiliza muchas de las herramientas reales que utilizaras en tu carrera de Rust. De hecho, al trabajar con cualquier projecto existente, puedes usar los siguientes comandos para ver el codigo usando Git, ir al directorio del projecto y armarlo:

$ git clone example.org/someproject
$ cd someproject
$ cargo build

Para mas informacion sobre Cargo, chequeen <a>su documentacion</a>. //missing   link to documentation

            /  /  / Resumen /  /  /

Ya estas bien encaminado en tu aventura con Rust! En este capitulo, aprendiste como:

-Instalar la ultima version de Rust usando "rustup".
-Actualizar a una nueva version de Rust.
-Abrir documentacion instalada localmente.
-Escribir y correr un programa "Hello, World!" de manera directa usando "rustc".
-Crear y correr un programa usando las convenciones de Cargo. 

Este es un momento genial para construir un programa mas substancial para acostumbrarte a leer y escribir codigo en Rust. Asi que, en el capitulo 2, construiremos el programa de un juego de adivinanzas. Si prefieres aprender como funcionan los conceptos comunes de la programacion en Rust, te recomiendo ir al capitulo 3 y luego retornar al capitulo 2.
