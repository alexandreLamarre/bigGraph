#include "../graph.h"

/**
 * Flexible Edge datatype
 * @param edgeData<char, std::any>: map from string to dynamic data:
 *        edgeData['s'] indicates start point(s) data 
 *        edgeData['e'] indicates end point(s) data
 *        edgeData['w] indicates edge weight
 *        edgeData['f'] indicates edge weight function
 *        edgeData['q] indicates quatum eigenfunctor of an edge
 *  
 * @param type : GraphType: one of simple, pseudo, multi, hyper, 
 *               uniformhyper
 * @param subTypes :  GraphSubType one of none, directed, weighted, 
 *                    functional, quantum
 **/ 
Edge::Edge(std::map<char, std::any> edgeData, 
             GraphType type, std::vector<GraphSubType> subTypes){
    
}

/**
 * @returns : the startpoint(s) data of the edge
 **/
std::any Edge::getStart(){
    if(data.find('s') == data.end()){ //edge has no startpoint data
        return -1; //TODO: return or throw more appropriate output
    } else{ 
        return data['s'];
    }
}

/**
 * @returns : the endpoint(s) data of the edge
 **/ 
std::any Edge::getEnd(){
    if(data.find('e') == data.end()){ //edge has no endpoint datas
        return -1; //TODO: return or throw more appropriate output
    } else{
        return data['e'];
    }
}

/**
 * @returns : the weight(s) of the edge
 **/ 
std::any Edge::getWeight(){
    if(data.find('w') == data.end()){ //edge has no endpoint datas
        return -1; //TODO: return or throw more appropriate output
    } else{
        return data['w'];
    }
}

/**
 * @returns : the functional of the edge
 */ 
std::any Edge::getFunc(){
    if(data.find('f') == data.end()){ //edge has no endpoint datas
        return -1; //TODO: return or throw more appropriate output
    } else{
        return data['f'];
    }
} 

/**
 * @returns : the quantum eigenfunctor of the edge
 **/
std::any Edge::getQuantum(){
    if(data.find('q') == data.end()){ //edge has no endpoint datas
        return -1; //TODO: return or throw more appropriate output
    } else{
        return data['q'];
    }
} 

/**
 * Set new start endpoint(s) data for the edge.
 * @param newStart: new startpoint data for the edge
 **/ 
void Edge::setStart(std::any newStart){
    data['s'] = newStart;
}

/**
 * Set new endpoint(s) data for the edge.
 * @param newEnd : new endpoint data for the edge
 **/ 
void Edge::setEnd(std::any newEnd){
    data['e'] = newEnd;
}

/**
 * Set new weight(s) data for the edge.
 * @param newWeight: new weight(s) for the edge
 **/ 
void Edge::setWeight(std::any newWeight){
    data['w'] = newWeight;
} 

/**
 * Set new functional for the edge
 * @param newFunc: new functional for the edge
 **/ 
void Edge::setFunc(std::any newFunc){
    data['f'] = newFunc;
}

/**
 * Set new quantum eigenfunctor for the edge
 **/ 
void Edge::setQuantum(std::any newEdge){
    data['q'] = newEdge;
}