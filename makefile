RUSTC = rustc
CHANGEDIR_VARIABLES = cd variables
CHANGEDIR_IFELSE = cd if

all: variables if-else

variables: variables1.rs variables2.rs variables3.rs
	
variables1.rs:
	${CHANGEDIR_VARIABLES} && ${RUSTC} variables1.rs 

variables2.rs:
	${CHANGEDIR_VARIABLES} && ${RUSTC} variables2.rs

variables3.rs:
	${CHANGEDIR_VARIABLES} && ${RUSTC} variables3.rs && cd..

if-else:
	${CHANGEDIR_IFELSE} && ${RUSTC} if-else.rs

clean: 
	rm **/*.pdb
	rm **/*.exe
