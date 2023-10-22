use crate::core::SparseSet;

#[test]
fn new_as_expected() {
    let sparse = SparseSet::<u64>::new();

    assert_eq!(sparse.data_len(), 0);
    assert_eq!(sparse.data_capacity(), 0);
}

#[test]
fn insert_as_expected() {
    let expected_value = 100;
    let mut set = SparseSet::new();
    let id = set.insert(100);

    assert_eq!(set.data_len(), 1);
    assert_eq!(set[id], expected_value);
}

#[test]
fn remove_as_expected() {
    let mut set = SparseSet::new();
    let id0 = set.insert(1);
    let id1 = set.insert(2);
    let id2 = set.insert(3);
    let id3 = set.insert(4);

    assert_eq!(set.remove(id1), 2);
    assert_eq!(set.data_len(), 3);
    assert!(set.has_sparse_index(id0));
    assert!(set.has_sparse_index(id1) == false);
    assert!(set.has_sparse_index(id2));
    assert!(set.has_sparse_index(id3));
}

#[test]
fn remove_id_recycle() {
    let mut set = SparseSet::new();
    set.insert(1);
    let id1 = set.insert(2);
    set.insert(3);

    set.remove(id1);

    let new_id = set.insert(2);

    assert_eq!(new_id, id1);
    assert_eq!(set[new_id], 2);
}