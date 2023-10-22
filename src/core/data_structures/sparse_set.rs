use std::ops::{Deref, DerefMut, Index, IndexMut};

/// The sparse set data structure allows to get the data by two memory access patterns:
/// * Random access - indexing in the structure is tracking by the "sparse" index - it has
/// permanent value, that recycles only after value will be removed from the set. The sparse
/// indicies can contain gaps between consequent elements that reference to the exisiting data
/// * Linear access - data in the structure is tracking by the "dense" index - it's tightly packed
/// in the memory and can be requested for iteration for example.
///
/// So when you need to get the data by a unique identifier - you are using a sparse index
/// (default `Index` implementation). When you need to process data linearly - you are
/// using the `data` slice or an iterator.
/// The `SparseSet` utilizes swap-remove algorithm to keep data tightly packed and uses 
/// free list to the fast binary search for freed sparse indices (aka sparse gaps)
pub struct SparseSet<T> {
    pub sparse: Vec<usize>,
    pub dense: Vec<usize>,
    pub dense_data: Vec<T>,
    pub free_sparse: Vec<usize>,
}

impl<T> SparseSet<T> {
    pub fn new() -> SparseSet<T> {
        SparseSet {
            sparse: Vec::new(),
            dense: Vec::new(),
            dense_data: Vec::new(),
            free_sparse: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> SparseSet<T> {
        SparseSet {
            sparse: Vec::with_capacity(capacity),
            dense: Vec::with_capacity(capacity),
            dense_data: Vec::with_capacity(capacity),
            free_sparse: Vec::new(),
        }
    }

    pub fn with_advance_capacity(
        sparse_capacity: usize,
        dense_capacity: usize,
        free_list_capacity: usize,
    ) -> SparseSet<T> {
        SparseSet {
            sparse: Vec::with_capacity(sparse_capacity),
            dense: Vec::with_capacity(dense_capacity),
            dense_data: Vec::with_capacity(dense_capacity),
            free_sparse: Vec::with_capacity(free_list_capacity)
        }
    }

    /// Adds the `value` to the set and returns its sparse index
    pub fn insert(&mut self, value: T) -> usize {
        self.dense_data.push(value);

        let dense_idx = self.dense_data.len() - 1;
        let sparse_idx = match self.free_sparse.pop() {
            Some(sparse_idx) => {
                self.sparse[sparse_idx] = dense_idx;
                sparse_idx
            }
            None => {
                self.sparse.push(dense_idx);
                self.sparse.len() - 1
            }
        };

        self.dense.push(sparse_idx);
        sparse_idx
    }

    /// Removes the value from the set and returns it
    pub fn remove(&mut self, sparse_index: usize) -> T {
        let dense_idx = self.sparse[sparse_index];
        let last_dense_idx = self.dense.len() - 1;
        let last_dense_to_sparse = self.dense[last_dense_idx];

        self.sparse.swap(sparse_index, last_dense_to_sparse);
        self.dense.swap(dense_idx, last_dense_idx);
        self.dense_data.swap(dense_idx, last_dense_idx);

        if sparse_index < self.sparse.len() - 1 {
            match self.free_sparse.binary_search(&sparse_index) {
                Ok(_) => panic!("The sparse index {sparse_index} is already freed!"),
                Err(idx) => self.free_sparse.insert(idx, sparse_index),
            }
        }

        self.dense_data.remove(last_dense_idx)
    }

    pub fn has_sparse_index(&self, sparse_index: usize) -> bool {
        if sparse_index >= self.sparse.len() {
            return false;
        }

        match self.free_sparse.binary_search(&sparse_index) {
            Ok(_) => false,
            Err(_) => true,
        }
    }

    pub fn data_len(&self) -> usize {
        self.dense_data.len()
    }

    pub fn data_capacity(&self) -> usize {
        self.dense_data.capacity()
    }

    pub fn data(&self) -> &[T] {
        &self.dense_data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.dense_data
    }
}

impl<T> Index<usize> for SparseSet<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.dense_data[self.sparse[index]]
    }
}

impl<T> IndexMut<usize> for SparseSet<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.dense_data[self.sparse[index]]
    }
}

impl<T> Deref for SparseSet<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.dense_data
    }
}

impl<T> DerefMut for SparseSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dense_data
    }
}
