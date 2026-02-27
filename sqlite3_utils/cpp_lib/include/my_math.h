#ifndef MY_MATH_H
#define MY_MATH_H

typedef struct {
    int x;
    int y;
    const char* label;
} Point;

// C Function: Calculate the distance from a point to the origin
float calculate_distance(Point p);

// Simple function to add two integers
int fast_add(int a, int b);

#endif
