pub mod sparse_csr;
pub mod sparse_ll;

#[derive(Debug, Default)]
pub struct Graph {}

impl Graph {}

impl ToString for Graph {
    fn to_string(&self) -> String {
        todo!("Implement me!")
    }
}

impl Iterator for Graph {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement me!")
    }
}
