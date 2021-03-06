#include <iostream>
#include <any>
#include <map>
#include <vector>
#include <string>

/**
 * Specifies how the graph should be built.
 * @attr random: generates a graph 
 *               randomly based on the additional parameters specified
 * @attr file: generates a graph based on the file uploaded
 * @attr request: generates a graph based on an api request
 **/
enum class GraphConstructor{
    random, 
    file,
    request
};

/**
 * Specifies the Graph Type : these are non compatible types which 
 *                            specify how the data relates to each other
 * 
 * @attr simple : simple graph one-to-one connections between nodes
 * @attr pseudo : simple graph where nodes can connect to each other
 * @attr multi : graph where multiple one-to-one connections can exit between nodes
 * @attr hyper : graph where edges represent many-to-many connections between nodes
 * @attr uniformhyper : graph where all edges are k-to-k connections between nodes
 **/ 
enum class GraphType{
    simple,
    pseudo,
    multi,
    hyper,
    uniformhyper,
};

/**
 * Specifies the Graph Property : these represent overall graph
 *                                properties specific to a graph definition 
 * @attr general : no specific property
 * @attr connected : you can reach any node from any node
 * @attr cycle : 
 **/ 
enum class GraphProperty{
    general,
    connected,
    cycle
};

/**
 * Specified Graph SubType : these represent the type of 'evalution metrics' or flow
 *                           that underly graph edges
 * @attr none : no special metric, edges represent a binary connection
 * @attr directed : edges are directed a->b, b->a
 * @attr weighted : edges have a value that determined their strength
 * @attr functional : edges have a functional evaluation metric
 * @attr quantum : edges are augmented with quantum network eigen functors
 **/
enum class GraphSubType{
    none, 
    directed, 
    weighted,
    functional,
    quantum
};

/**
 * Vertex/node data structure.
 * A vertex contain a collection of related information
 **/
class Vertex{
    public:
        Vertex(float x, float y, float z);
        float getX();
        float getY();
        float getZ();
        void setX(float x);
        void setY(float y);
        void setZ(float z);
    private:
        float x;
        float y;
        float z;
};

/**
 * Edge/node data structure.
 * An edge represents relations between nodes.
 **/
class Edge{
    public:
        Edge(std::map<char, std::any> edgeData, 
             GraphType type, std::vector<GraphSubType> subTypes);
        std::any getStart();
        std::any getEnd();
        std::any getWeight();
        std::any getFunc();
        std::any getQuantum();
        void setStart(std::any);
        void setEnd(std::any);
        void setWeight(std::any);
        void setFunc(std::any);
        void setQuantum(std::any);
        std::map<char, std::any> data;
};

/**
 * Graph data structure.
 * A graph represents the way we can represent relational data
 **/
class Graph{
    public:
        Graph();
};

