#include <iostream>
#include <sys/time.h>

int64_t getCurrentTime() {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return tv.tv_sec * 1000 * 1000 + tv.tv_usec;
}

double det(int dim, bool debug) {

    double data[dim][dim];
    for (int i = 0; i < dim; i++) {
        for (int j = 0; j < dim; j++) {
            if (i == 0 || j == 0) data[i][j] = 1;
            else data[i][j] = data[i - 1][j] + data[i][j - 1];
        }
    }

    if (debug) {
        std::cout << "before diagonalize" << std::endl;
        for (int i = 0; i < dim; i++) {
            for (int j = 0; j < dim; j++) {
                printf("%.3f, ", data[i][j]);
            }
            printf("\n");
        }
    }

    int64_t ts = getCurrentTime();
    for (int k = 0; k < dim - 1; k++) {
        if (data[k][k] == 0) {
            int swapped = 0;
            // swap rows to make it no-zero
            for (int i = k + 1; i < dim; i++) {
                if (data[i][i] != 0) {
                    // swap_row: k, i
                    for (int j = 0; j < dim; j++) {
                        double tmp = data[k][j];
                        data[k][j] = data[i][j];
                        data[i][j] = tmp;
                    }
                    swapped = 1;
                    break;
                }
            }
            if (swapped == 0) {
                return 0;
            }
        }

        // this[k][k] will not equal zero in this area
        for (int i = k + 1; i < dim; i++) {
            // add_other_row: i, k, -self.data[i][k] / self.data[k][k]
            double multiple = -data[i][k] / data[k][k];
            for (int j = 0; j < dim; j++) {
                data[i][j] += multiple * data[k][j];
            }
        }
    }

    if (debug) {
        std::cout << "after diagonalize" << std::endl;
        for (int i = 0; i < dim; i++) {
            for (int j = 0; j < dim; j++) {
                printf("%.3f, ", data[i][j]);
            }
            printf("\n");
        }
    }

    double result = 1;
    for (int k = 0; k < dim; k++) {
        result *= data[k][k];
    }
    ts = getCurrentTime() - ts;
    printf("%3d-level pascal det cost %lli us, and value is %.8G\n", dim, ts, result);
    return result;
}

double det(int dim) {
    return det(dim, false);
}

int main() {
    double d = det(12, true);
    printf("%2d-level pascal det is %.8G\n", 12, d);

    for (int k = 1; k <= 128; k++) {
        double d = det(k);
        //printf("%2d-level pascal det is %.8G\n", k, d);
    }
    return 0;
}