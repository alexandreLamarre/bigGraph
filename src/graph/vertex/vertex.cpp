#include "../graph.h"

/**
 * Initializes a vertex with its coordinates
 * @param x: the x coordinate of the vertex
 * @param y: the y coordinate of the vertex
 * @param z: the z coordinate of the vertex (only used in 3D graphs)
 **/ 
Vertex::Vertex(float x, float y, float z){
    x = x;
    y = y;
    z = z;
}

/**
 * @returns float: the x coordinate of the vertex
 **/ 
float Vertex::getX(){
    return x;
}

/**
 * @returns float: the x coordinate of the vertex
 **/ 
float Vertex::getY(){
    return y;
}

/**
 * @returns float: the x coordinate of the vertex
 **/ 
float Vertex::getZ(){
    return z;
}
