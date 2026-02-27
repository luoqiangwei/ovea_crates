#include "my_math.h"
#include <math.h>

float calculate_distance(Point p) {
    return sqrtf(p.x * p.x + p.y * p.y);
}

int fast_add(int a, int b) {
    return a + b;
}
