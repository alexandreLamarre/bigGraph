#include "../graph/graph.h"

/**
 * Used for debugging tests
 * @attr: off : turn off debug messages
 * @attr: partial: only important/critical debug messages
 * @attr: on: all debug messages
 **/ 
enum class Debug{
    off,
    partial,
    on,
};


/**
 * @param debugType : int specifying off, partial, on
 * @returns float : the percentage of passed tests
 **/ 
float testGraph(Debug debugType);

/**
 * @param debugType : int specifying off, partial, on
 * @returns float : the percentage of passed tests
 **/ 
float testVertex(Debug debugType);

/**
 * @param debugType : int specifying off, partial, on
 * @returns float : the percentage of passed tests
 **/ 
float testEdge(Debug debugType);

// ================= test suite definitions ========
float testVertexConstructor(Debug debugType);
float testVertexSetters(Debug debugType);
