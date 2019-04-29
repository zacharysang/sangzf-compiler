#include <stdio.h>
#include <string.h>

/* To build:
    * gcc -c -Wall -Werror -fpic builtins.c
    * gcc -shared -o builtins.so builtins.o

*/

// Boolean functions
void putBool(int val) {
  if (val != 0) {
    printf("true\n");
  } else {
    printf("false\n");
  }
}

int getBool() {
  const int MAX_LEN = 1;

  char input[MAX_LEN];
  fgets(input, MAX_LEN, stdin);

  if (strncmp("1", input, MAX_LEN) == 0) {
    return 1;
  } else if (strncmp("0", input, MAX_LEN) == 0) {
    return 0;
  } else {
    printf("Invalid boolean provided. Defaulting to FALSE");
    return 0;
  }
}

// Integer functions
void putInteger(int* val) {
  printf("%d\n", *val);
}

int getInteger() {
  const int MAX_LEN = 256;
  char input[MAX_LEN];
  fgets(input, MAX_LEN, stdin);
	
  int output[1];
  sscanf(input, "%d", output);

  return *output;
}

// Float functions

// String functions
