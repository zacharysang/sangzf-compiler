#include <stdio.h>
#include <string.h>

/* 
  To build:
    * gcc -c -Wall -Werror -fpic builtins.c
    * gcc -shared -o builtins.so builtins.o
*/

// Boolean functions
int putbool(int val) {
  if (val != 0) {
    printf("true\n");
  } else {
    printf("false\n");
  }
  
  return 0;
}

int getbool() {
  const int MAX_LEN = 2;

  char input[MAX_LEN];
  fgets(input, MAX_LEN, stdin);

  if (strncmp("1", input, MAX_LEN) == 0) {
    return 1;
  } else if (strncmp("0", input, MAX_LEN) == 0) {
    return 0;
  } else {
    printf("Invalid boolean provided: '%s'. Defaulting to FALSE\n", input);
    return 0;
  }
}

// Integer functions
int putinteger(int val) {
  printf("%d\n", val);
  
  return 0;
}

int getinteger() {
  int output;
  scanf("%d*[^\n]", &output);

  return output;
}

// Float functions
int putfloat(float val) {
  printf("%f\n", val);
  
  return 0;
}

float getfloat() {
  float output;
  scanf("%f*[^\n]", &output);
  
  return output;
}

// String functions
/*
int putstring(char* val) {
  printf("%s\n", val);
  
  return 0;
}

char* getstring() {
  const int MAX_LEN = 512;
  
  char buff[MAX_LEN];
  fgets(buff, MAX_LEN, stdin);
  
  return buff;
}
*/