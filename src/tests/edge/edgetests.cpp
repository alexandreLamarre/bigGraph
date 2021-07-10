#include <iostream>
#include "../graphTests.h"

float testEdge(Debug debugType){
    float cur = 0.0f;
    float total = 2.0f;

    cur += testEdgeConstructor(debugType);
    cur += testEdgeSetters(debugType);
    return cur/total;
}

float testEdgeConstructor(Debug debugType){
    float cur = 0.0f;
    float total = 4.0f;

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

    if(start == 0){
        cur ++;
    } else{
        if(debugType >= Debug::partial){
            std::cout << "Expected edge start to be 0, Got : " << start << std::endl; 
        }
    }

    if(end == 1){
        cur ++;
    } else{
        if(debugType >= Debug::partial){
            std::cout << "Expected edge end to be 0, Got : " << end << std::endl;
        }
    }

    Edge e2 = Edge(
        std::map<char, std::any>{
            {'s', std::vector<int>{1,2,3,4}},
            {'e', std::vector<int>{4,5,6,7}}
        },
        GraphType::hyper,
        std::vector<GraphSubType> {GraphSubType::none}
    );
    std::vector<int> starts = std::any_cast<std::vector<int>>(e2.getStart());
    std::vector<int> ends = std::any_cast<std::vector<int>>(e2.getEnd());

    if(starts[0] == 1 && starts[1] == 2 && starts[2] == 3 && starts[3] == 4){
        cur ++;
    } else{
        if(debugType >= Debug::partial){
            std::cout << "Expected edge starts to be 1, 2, 3, 4, Got : " << starts[0] << " , " \
                    << starts[1] << " , " << starts[2] << " , " << starts[3] << std::endl;
        }
    }

    if(ends[0] == 4 && ends[1] == 5 && ends[2] == 6 && ends[3] == 7){
        cur ++;
    } else{
        if(debugType >= Debug::partial){
            std::cout << "Expected edge starts to be 1, 2, 3, 4, Got : " << ends[0] << " , " \
                    << ends[1] << " , " << ends[2] << " , " << ends[3] << std::endl;
        }
    }

    return cur == total? 1.0f : 0.0f;

}

float testEdgeSetters(Debug debugType){
    float cur = 0.0f;
    float total = 4.0f;

    Edge e = Edge(
        std::map<char, std::any>{
            {'s', 0},
            {'e', 1}
        },
        GraphType::simple,
        std::vector<GraphSubType> {GraphSubType::none}
    );

    e.setStart(4);
    int start = std::any_cast<int>(e.getStart());
    if(start == 4){
        cur ++;
    } else{
        if(debugType >= Debug::partial){
            std::cout << "Expected edge start to be 4, Got : " << start << std::endl;
        }
    }

    e.setEnd(12);
    e.setEnd(79);
    int end = std::any_cast<int>(e.getEnd());
    if(end == 79){
        cur ++;
    } else{
        if(debugType >= Debug::partial){
            std::cout << "Expected edge end to be 79, Got : " << end << std::endl;
        }
    }


    Edge e2 = Edge(
        std::map<char, std::any>{
            {'s', std::vector<int>{1,2,3,4}},
            {'e', std::vector<int>{4,5,6,7}}
        },
        GraphType::hyper,
        std::vector<GraphSubType> {GraphSubType::none}
    );

    std::vector<int> newStarts = std::vector<int>{10,11,12,13};
    e2.setStart(newStarts);
    std::vector<int> starts2 = std::any_cast<std::vector<int>>(e2.getStart());
    if(starts2[0] == 10 && starts2[1] == 11 && starts2[2] == 12 && starts2[3] == 13){
        cur ++;
    } else{
        if(debugType >= Debug::partial){
            std::cout << "Expected edge starts to be 10, 11, 12, 13, Got : " << starts2[0] << " , " << starts2[1] << " , " << starts2[2] << " , " << starts2[3] << std::endl;
        }
    }

    std::vector<int> newEnds = std::vector<int>{25,26,227,228};
    e2.setEnd(newEnds);
    std::vector<int> ends2 = std::any_cast<std::vector<int>>(e2.getEnd());
    if(ends2[0] == 25 && ends2[1] == 26 && ends2[2] == 227 && ends2[3] == 228){
        cur++;
    } else{
        if(debugType >= Debug::partial){
            std::cout << "Expected edge ends to be 25, 26, 227, 228, Got : " << ends2[0] << ", " << ends2[1] << " , " << ends2[2] << " , " << ends2[3] << std::endl;       
        }
    }
    return cur == total? 1.0f: 0.0f;
}