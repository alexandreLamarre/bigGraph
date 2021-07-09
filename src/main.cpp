#include <iostream>
#include "tests/graphTests.h"

int main(){
    std::cout << "Hello bigGraph!" << std::endl;
    std::cout << "Vertex tests passed : " << testVertex(Debug::off) * 100 << "%"  << std::endl;
    std::cout << "Edge tests passed : " << testEdge(Debug::off) * 100 << "%"  << std::endl;
    std::cout << "Graph tests passed : " << testGraph(Debug::off) * 100 << "%"  << std::endl;
}