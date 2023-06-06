#ifndef _RUST_DEMO_H_
#define _RUST_DEMO_H_

char* fr_plus(char*, char*);
char* generate_params_1();
char* commit_1(char *srs, char *messages);
char* open_1(char *srs, char *message, int pos);
int verify_1(char *srs, char *commitment, char *message, int pos, char *witness);

#endif