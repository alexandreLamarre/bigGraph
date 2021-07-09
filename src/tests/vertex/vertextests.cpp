#include <iostream>
#include "../graphTests.h"


float testVertex(Debug debugType){
    float totalTests = 2.0f;
    float passedTests = 0.0f;

    passedTests += testVertexConstructor(debugType);
    passedTests += testVertexSetters(debugType);
    return passedTests/totalTests;
}

/**
 * Tests constructor, as well as getters and setters
 * 
 **/ 
float testVertexConstructor(Debug debugType){
    float count = 0.0; 
    float total = 3.0;

    Vertex v = Vertex(3.0, 4.0, 5.0);

    float xPos = v.getX();
    if(xPos == 3.0){
        count ++;
    }
    float yPos = v.getY();
    if(yPos == 4.0){
        count ++;
    }

    float zPos = v.getZ();
    if(zPos == 5.0){
        count ++;
    }

    if(debugType >= Debug::partial){
        std::cout << count << " / " << total << " tests passed for testVertexConstructors" << std::endl;
    }
    return (count == total) ? 1.0f : 0.0f;
}

float testVertexSetters(Debug debugType){
    float count = 0.0;
    float total = 3.0;

    Vertex v = Vertex(3.0, 4.0, 5.0);

    v.setX(12.0);
    float newXpos = v.getX();
    if(newXpos == 12.0f){
        count ++;
    } else{
        if(debugType >= Debug::partial){
            std :: cout << "Expected Vertex' x-coord to be 12.0, Got : " << newXpos << std::endl;
        }
    }

    v.setY(144.44f);
    float newYpos = v.getY();
    if(newYpos == 144.44f){
        count ++;
    }else{
        if(debugType >= Debug::partial){
            std :: cout << "Expected Vertex' y-coord to be 144.44, Got : " << newYpos << std::endl;
        }
    }

    v.setZ(313.6789);
    float newZpos = v.getZ();
    if(newZpos == 313.6789f){
        count ++;
    } else {
        if(debugType >= Debug::partial){
            std :: cout << "Expected Vertex' x-coord to be 313.6789, Got : " << newZpos << std::endl;
        }
    }

    if(debugType >= Debug::partial){
        std::cout << count << " / " << total << " tests passed for testVertexSetters" << std::endl;
    }

    return (count == total) ? 1.0f: 0.0f;
}