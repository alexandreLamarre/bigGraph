/*
    Compressed Sparse Row (CSR) graph representation.
*/

/// Allows locating a vertex's neighbors in O(1) time
pub struct CSRGraph<T> {
    // Stores values of vertices visited
    A: Vec<T>,
    // Of size M+1 (where the Matrix is MxN), and stores the cumulative number
    // of non-zero elements upto (not including) the i-th row.
    // It is defined by the recursive relation:
    //
    // IA[0] = 0
    // IA[i] = IA[i - 1] + number of non zero elemnts in the i-1 th row of the Matrix
    IA: Vec<usize>,
    //  Stores the column index of each vertex
    JA: Vec<usize>,
}

/// PCSR (Partitioned Compressed Sparse Row) graph representation.
///
/// GPU-friendly data structure derived from CSR
pub struct PCSRGraph {
    // TODO!
}
