#ifndef _RUST_DEMO_H_
#define _RUST_DEMO_H_

char* fr_plus(char*, char*);
char* generate_params(int);
char* commit(int, char *srs, char *messages);
char* open(int, char *srs, char *message, int pos);
int verify(int, char *srs, char *commitment, char *message, int pos, char *witness);

#endif