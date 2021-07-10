#include <iostream>
#include "../graphTests.h"

float testEdge(Debug debugType){
    float cur = 0.0f;
    float total = 12.0f;
    
    testEdgeConstructor(debugType);
    return 1.0f;
}

float testEdgeConstructor(Debug debugType){
    float cur = 0.0f;
    float total = 12.0f;

    //Simple edge
    Edge e = Edge(
        std::map<char, std::any>{
            {'s', 0},
            {'e', 1}
        },
        GraphType::simple,
        std::vector<GraphSubType> {GraphSubType::none}
    );
    int start = std::any_cast<int>(e.getStart());
    int end = std::any_cast<int>(e.getEnd());
    std::cout << "start : " <<  &start << std::endl;
    std::cout << "end : " << &end << std::endl;
}