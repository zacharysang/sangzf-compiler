runtime: runtime-clean
	cd src/builtins; gcc -c -Wall -Werror -fpic builtins.c;
runtime-clean:
	rm src/builtins/builtins.o src/builtins/builtins.so
	