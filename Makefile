program: program-clean runtime
	cargo run sample_programs/correct/$(NAME).src
	llc -filetype=obj $(NAME).bc
	gcc -o $(NAME) $(NAME).o src/builtins/builtins.o
	
program-clean:
	rm -f $(NAME).o $(NAME).bc $(NAME)

runtime: runtime-clean
	cd src/builtins; gcc -c -Wall -Werror -fpic builtins.c;
runtime-clean:
	rm -f src/builtins/builtins.o src/builtins/builtins.so
	