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

// Integer functions
int putinteger(int val) {
  printf("%d\n", val);
  
  return 0;
}

// Float functions
int putfloat(float val) {
  printf("%f\n", val);
  
  return 0;
}

int getbool() {
  int output;
  
  scanf("%d%*[^\n]", &output);

  if (output == 0) {
    return 0;
  } else {
    return 1;
  }
}


int getinteger() {
  int output;
  scanf("%d%*[^\n]", &output);

  return output;
}

float getfloat() {
  float output;
  scanf("%f%*[^\n]", &output);
  
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