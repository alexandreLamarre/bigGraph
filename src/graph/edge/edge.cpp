#include "../graph.h"

Edge::Edge(int newStart, int newEnd){
    start = newStart;
    end = newEnd;
}

/**
 * @returns : the start index of the edge 
 **/
int Edge::getStart(){
    return start;
}

/**
 * @returns : the end index of the edge
 **/ 
int Edge::getEnd(){
    return end;
}

/**
 * Set new start endpoint vertex for the edge.
 * @param newStart: new index for the start vertex of the edge
 **/ 
void Edge::setStart(int newStart){
    start = newStart;
}

/**
 * Set new endpoint vertex index for the edge.
 * @param newEnd : new index for the end vertex of the edge
 **/ 
void Edge::setEnd(int newEnd){
    end = newEnd;
}