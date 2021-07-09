#include <iostream>

/**
 * Vertex/node data structure.
 * A vertex contain a collection of related information
 **/
class Vertex{
    public:
        Vertex();
};

/**
 * Edge/node data structure.
 * An edge represents relations between nodes.
 **/
class Edge{
    public:
        Edge();
};

/**
 * Graph data structure.
 * A graph represents the way we can represent relational data
 **/
class Graph{
    public:
        Graph();
};

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
enum class Property{
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
enum class SubType{
    none, 
    directed, 
    weighted,
    functional,
    quantum
};