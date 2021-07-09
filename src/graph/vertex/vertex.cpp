#include "../graph.h"
#include <iostream>

/**
 * Initializes a vertex with its coordinates
 * @param newX: the x coordinate of the vertex
 * @param newY: the y coordinate of the vertex
 * @param newZ: the z coordinate of the vertex (only used in 3D graphs)
 **/ 
Vertex::Vertex(float newX, float newY, float newZ){
    x = newX;
    y = newY;
    z = newZ;
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

/**
 * Setter for x coordinate of a vertex
 * @param newX: the new x coordinate of a vertex
 **/ 
void Vertex::setX(float newX){
    x = newX;
}

/**
 * Setter for y coordinate of a vertex
 * @param newY: the new y coordinate of a vertex
 **/ 
void Vertex::setY(float newY){
    y = newY;
}

/**
 * Setter for z coordinate of a vertex
 * @param newZ: the new z coordinate of a vertex
 **/ 
void Vertex::setZ(float newZ){
    z = newZ;
}
