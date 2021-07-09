#include <iostream>
#include "tests/graphTests.h"

int main(){
    std::cout << "Hello bigGraph!" << std::endl;
    float vertexPassed = testVertex(Debug::off) * 100;
    float edgePassed = testEdge(Debug::off) * 100;
    float graphPassed = testGraph(Debug::off) * 100;
    std::cout << "Vertex tests passed : " << vertexPassed << "%"  << std::endl;
    std::cout << "Edge tests passed : " << edgePassed << "%"  << std::endl;
    std::cout << "Graph tests passed : " << graphPassed << "%"  << std::endl;
}