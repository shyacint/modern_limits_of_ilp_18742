#include <stdio.h>

#define N 5  // Vector size

void saxpy(float alpha, float X[N], float Y[N]) {
    for (int i = 0; i < N; i++) {
        Y[i] = alpha * X[i] + Y[i];
    }
}

void printVector(float V[N]) {
    for (int i = 0; i < N; i++) {
        printf("%.2f ", V[i]);
    }
    printf("\n");
}

int main() {
    float alpha = 2.0;
    float X[N] = {1.0, 2.0, 3.0, 4.0, 5.0};
    float Y[N] = {5.0, 4.0, 3.0, 2.0, 1.0};

    printf("Original Y: ");
    printVector(Y);

    saxpy(alpha, X, Y);

    printf("Updated Y: ");
    printVector(Y);

    return 0;
}
