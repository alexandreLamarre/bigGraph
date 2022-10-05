/*
Basic Sparse Graph Datastructures
using HashMaps or LinkedLists
*/

use std::collections::HashMap;
use std::collections::LinkedList;
use std::error::Error;

/// 2D array is used to represent a sparse matrix in which there are
/// three rows named as
/// - Row : index of row, where non-zero element is present
/// - Column  : index of column where non-zero element is present
/// - Value : value of the non-zero  located at index (row,col)

type Coordinate = (usize, usize, usize);
#[derive(Debug, Default, Clone)]
pub struct SparseArrayGraph<T> {
    pub contents: HashMap<Coordinate, T>,
}

impl<T> SparseArrayGraph<T>
where
    T: Default,
    T: Clone,
    T: PartialEq,
{
    // from 2d array of values T
    fn new(input: Vec<Vec<T>>) -> Result<Self, Box<dyn Error>> {
        let mut ret: SparseArrayGraph<T> = SparseArrayGraph {
            contents: HashMap::new(),
        };
        let cached_default = T::default();
        for (irow, row) in input.iter().enumerate() {
            for (icol, col) in row.iter().enumerate() {
                if col != &cached_default {
                    ret.contents.insert((irow, icol, 0), col.clone());
                }
            }
        }
        Ok(ret)
    }
}

#[derive(Debug, Default, Clone)]
pub struct SparseLLGraph<T> {
    // zipped coordinates with value T
    pub contents: LinkedList<(usize, usize, usize, T)>,
}

impl<T> SparseLLGraph<T>
where
    T: Clone,
    T: Default,
    T: PartialEq,
{
    // from 2d array of values T
    fn new(input: Vec<Vec<T>>) -> Result<Self, Box<dyn Error>> {
        let mut ret: SparseLLGraph<T> = SparseLLGraph {
            contents: LinkedList::new(),
        };
        let cached_default = T::default();
        for (irow, row) in input.iter().enumerate() {
            for (icol, col) in row.iter().enumerate() {
                if col != &cached_default {
                    ret.contents.push_back((irow, icol, 0, col.clone()));
                }
            }
        }
        Ok(ret)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SparseNestedLLGraph<T> {
    contents: Vec<LinkedList<(usize, usize, usize, T)>>,
}

impl<T> SparseNestedLLGraph<T>
where
    T: Clone,
    T: Default,
    T: PartialEq,
{
    // from 2d array of values T
    fn new(input: Vec<Vec<T>>) -> Result<Self, Box<dyn Error>> {
        let mut ret: SparseNestedLLGraph<T> = SparseNestedLLGraph {
            contents: Vec::new(),
        };

        if input.len() == 0 {
            ret.contents.push(LinkedList::new());
        }

        let cached_default = T::default();
        for (irow, row) in input.iter().enumerate() {
            let mut ll = LinkedList::new();
            for (icol, col) in row.iter().enumerate() {
                if col != &cached_default {
                    ll.push_back((irow, icol, 0, col.clone()));
                }
            }
            ret.contents.push(ll);
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![], 0)]
    #[case(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9)]
    #[case(vec![vec![0, 0 , 0], vec![0, 0, 0], vec![0, 0, 0]], 0)]
    #[case(vec![vec![1, 0 , 0], vec![0, 1, 0], vec![0, 0, 1]], 3)]
    fn test_constructor_sparse_array(#[case] input: Vec<Vec<i32>>, #[case] expected_len: usize) {
        // empty graph should work
        let graph = SparseArrayGraph::<i32>::new(input);
        assert!(graph.is_ok());
        let real_graph = graph.unwrap();
        assert_eq!(real_graph.contents.len(), expected_len);
    }

    #[rstest]
    #[case(vec![], 0)]
    #[case(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9)]
    #[case(vec![vec![0, 0 , 0], vec![0, 0, 0], vec![0, 0, 0]], 0)]
    #[case(vec![vec![1, 0 , 0], vec![0, 1, 0], vec![0, 0, 1]], 3)]
    fn test_constructor_sparse_ll(#[case] input: Vec<Vec<i32>>, #[case] expected_len: usize) {
        // empty graph should work
        let graph = SparseLLGraph::<i32>::new(input);
        assert!(graph.is_ok());
        let real_graph = graph.unwrap();
        assert_eq!(real_graph.contents.len(), expected_len);
    }

    #[rstest]
    #[case(vec![], vec![0])]
    #[case(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], vec![3,3,3])]
    #[case(vec![vec![0, 0 , 0], vec![0, 0, 0], vec![0, 0, 0]], vec![0,0,0])]
    #[case(vec![vec![1, 0 , 0], vec![0, 1, 0], vec![0, 0, 1]], vec![1,1,1])]
    fn test_constructor_nested_ll(#[case] input: Vec<Vec<i32>>, #[case] expected_len: Vec<usize>) {
        // empty graph should work
        let graph = SparseNestedLLGraph::<i32>::new(input);
        assert!(graph.is_ok());
        let real_graph = graph.unwrap();
        for (i, ll) in real_graph.contents.iter().enumerate() {
            assert_eq!(ll.len(), expected_len[i]);
        }
    }
}

#[cfg(test)]
mod perf_tests {
    use super::*;
    // TODO!
}
