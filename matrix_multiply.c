#include <stdio.h>

#define ROWS 3
#define COLS 3

void multiplyMatrices(int firstMatrix[ROWS][COLS], int secondMatrix[ROWS][COLS], int resultMatrix[ROWS][COLS]) {
    for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
            resultMatrix[i][j] = 0;
            for (int k = 0; k < COLS; k++) {
                resultMatrix[i][j] += firstMatrix[i][k] * secondMatrix[k][j];
            }
        }
    }
}

void printMatrix(int matrix[ROWS][COLS]) {
    for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
            printf("%d ", matrix[i][j]);
        }
        printf("\n");
    }
}

int main() {
    int firstMatrix[ROWS][COLS] = {
        {1, 2, 3},
        {4, 5, 6},
        {7, 8, 9}
    };

    int secondMatrix[ROWS][COLS] = {
        {9, 8, 7},
        {6, 5, 4},
        {3, 2, 1}
    };

    int resultMatrix[ROWS][COLS];

    multiplyMatrices(firstMatrix, secondMatrix, resultMatrix);

    printf("Resultant Matrix:\n");
    printMatrix(resultMatrix);

    return 0;
}
