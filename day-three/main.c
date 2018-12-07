#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <regex.h>
#include <sys/types.h>

#define fromLeft "[0-9]*,"
#define fromTop ",[0-9]*"
#define cutWidth "[0-9]*x"
#define cutHeight "x[0-9]*"
#define Numbers "[0-9]*"

#define fabricWidth 1000
#define fabricHeight 1000

struct File {
    char* buffer;
    long fileSize;
};

struct Square {
    int leftIndent;
    int topIndent;
    int width;
    int height;
};

struct File readFile(char* filename) ;

int **buildFabric(int width, int height);

struct Square buildSquare(char* line);

void printFabric(int **fabric, int width, int height);

void printFile(struct File);

char* match(char* pattern, char* string) ;

struct Square * buildSquares(struct File file) ;

int main() {

    struct File file = readFile("../inputs");
    int **fabric = buildFabric(fabricWidth, fabricHeight);
    struct Square *squares = buildSquares(file);
    
    return 0;
}

void printFabric(int **fabric, int width, int height) {
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            printf("%d", fabric[i][j]);
        }
        printf("%c", '\n');
    }
}

int **buildFabric(int width, int height) {

    int **fabric = malloc(height*sizeof(int *));

    for(int i = 0 ; i < height ; i++) {
        fabric[i] = malloc(width * sizeof(int));
        for (int j = 0; j < width; j++) {
            fabric[i][j] = 0;
        }
    }

    return fabric;
}

struct Square * buildSquares(struct File file) {
    struct Square * squares = malloc(file.fileSize * sizeof(struct Square));

    for (int i = 0; i < file.fileSize; i++) {
        squares[i] = buildSquare(&file.buffer[i]);
    }

    return squares;
}

struct Square buildSquare (char* line) {
    struct Square square;

    square.leftIndent = atoi(match(Numbers, match(fromLeft, line)));
    square.topIndent = atoi(match(Numbers, match(fromTop, line)));
    square.width = atoi(match(Numbers, match(cutWidth, line)));
    square.height = atoi(match(Numbers, match(cutHeight, line)));

    return square;
}

char* match(char* pattern, char* string) {
    int len;
    int res;
    char* result= (char *) malloc(BUFSIZ);
    regex_t preg;
    regmatch_t pmatch[10];

    res = regcomp(&preg, pattern, REG_EXTENDED);
    res = regexec(&preg, string, 10, pmatch, REG_NOTBOL);

    len = pmatch[0].rm_eo - pmatch[0].rm_so;
    memcpy(result, string + pmatch[0].rm_so, len);
    result[len] = 0;
    return &result[0];
}

struct File readFile(char* filename) {
    FILE * inputFile;
    size_t result;
    struct File file;

    inputFile = fopen(filename, "r");

    if (inputFile==NULL) {fputs ("File error",stderr); exit (1);}

    fseek (inputFile , 0 , SEEK_END);
    file.fileSize = ftell (inputFile);
    rewind (inputFile);

    file.buffer = (char*) malloc (sizeof(char)*(file.fileSize + 1));
    if (file.buffer == NULL) {fputs ("Memory error",stderr); exit (2);}

    result = fread (file.buffer, 1, (size_t) file.fileSize, inputFile);
    if (result != file.fileSize) {fputs ("Reading error",stderr); exit (3);}

    return file;
}

void printFile(struct File file) {
    for (int i = 0; i < file.fileSize; i++) {
        printf("%c", file.buffer[i]);
    }
}