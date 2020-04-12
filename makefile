RUSTC = rustc
CHANGEDIR_VARIABLES = cd variables
CHANGEDIR_IFELSE = cd if
CHANGEDIR_BUCLES = cd bucles
CHANGEDIR_FUNCIONES= cd funciones
CHANGEDIR_GESTMEMORIA = cd gestion-memoria

all: variables if-else bucles funciones gestion-memoria

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

gestion-memoria: gestion-memoria1.rs

gestion-memoria1.rs:
	${CHANGEDIR_GESTMEMORIA} && ${RUSTC} gestion-memoria1.rs

clean: 
	rm **/*.pdb
	rm **/*.exe
