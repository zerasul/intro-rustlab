RUSTC = rustc
CARGO = cargo build
CHANGEDIR_VARIABLES = cd variables
CHANGEDIR_IFELSE = cd if
CHANGEDIR_BUCLES = cd bucles
CHANGEDIR_FUNCIONES= cd funciones
CHANGEDIR_GESTMEMORIA = cd gestion-memoria
CHANGEDIR_CADENAS = cd cadenas
CHANGEDIR_ARRAYS= cd arrays-tuplas
CHANGEDIR_STRUCTS= cd structs-enums
CHANGEDIR_OPTION= cd option-result
CHANGEDIR_MODULES = cd modulos
CHANGEDIR_CARGO = cd cargo/miproyecto

all: variables if-else bucles funciones gestion-memoria cadenas arrays-tuplas structs-enums option-result modulos cargo

variables: variables1.rs variables2.rs variables3.rs
	
variables1.rs:
	${CHANGEDIR_VARIABLES} && ${RUSTC} variables1.rs 

variables2.rs:
	${CHANGEDIR_VARIABLES} && ${RUSTC} variables2.rs

variables3.rs:
	${CHANGEDIR_VARIABLES} && ${RUSTC} variables3.rs

if-else:
	${CHANGEDIR_IFELSE} && ${RUSTC} if-else.rs

bucles: bucles1.rs bucles2.rs

bucles1.rs:
	${CHANGEDIR_BUCLES} && ${RUSTC} bucles1.rs

bucles2.rs:
	${CHANGEDIR_BUCLES} && ${RUSTC} bucles2.rs

funciones: funciones1.rs

funciones1.rs:
	${CHANGEDIR_FUNCIONES} && ${RUSTC} funciones1.rs

gestion-memoria: gestion-memoria1.rs gestion-memoria2.rs

gestion-memoria1.rs:
	${CHANGEDIR_GESTMEMORIA} && ${RUSTC} gestion-memoria1.rs

gestion-memoria2.rs:
	${CHANGEDIR_GESTMEMORIA} && ${RUSTC} gestion-memoria2.rs

cadenas: cadenas1.rs cadenas2.rs

cadenas1.rs:
	${CHANGEDIR_CADENAS} && ${RUSTC} cadenas1.rs

cadenas2.rs:
	${CHANGEDIR_CADENAS} && ${RUSTC} cadenas2.rs

arrays-tuplas: arrays-tuplas1.rs arrays-tuplas2.rs

arrays-tuplas1.rs:
	${CHANGEDIR_ARRAYS} && ${RUSTC} arrays-tuplas1.rs

arrays-tuplas2.rs:
	${CHANGEDIR_ARRAYS} && ${RUSTC} arrays-tuplas2.rs

structs-enums: structs-enums1.rs structs-enums2.rs

structs-enums1.rs:
	${CHANGEDIR_STRUCTS} && ${RUSTC} structs-enums1.rs
structs-enums2.rs:
	${CHANGEDIR_STRUCTS} && ${RUSTC} structs-enums2.rs

option-result: option-result1.rs option-result2.rs

option-result1.rs:
	${CHANGEDIR_OPTION} && ${RUSTC} option-result1.rs

option-result2.rs:
	${CHANGEDIR_OPTION} && ${RUSTC} option-result2.rs

modulos: modulos1.rs

modulos1.rs:
	${CHANGEDIR_MODULES} && ${RUSTC} modulos1.rs

cargo: cargo1

cargo1:
	${CHANGEDIR_CARGO} && ${CARGO}

clean: 
	rm **/*.pdb
	rm **/*.exe
