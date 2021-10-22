crate::use_native_or_external!(Vec);
use super::Bytes;
use bumpalo::Bump;

#[test]
fn test_new() {
    let bump = Bump::new();
    let bytes = Bytes::new(&bump, bump_vec![in &bump; 1, 2, 3]);
    drop(bytes);
}

#[test]
fn test_as_raw() {
    let bump = Bump::new();
    let bytes = Bytes::new(&bump, bump_vec![in &bump; 1, 2, 3]);

    assert_eq!(bytes.as_raw(), &bump_vec![in &bump; 1, 2, 3]);
    drop(bytes);
}

#[test]
fn test_into_raw() {
    let bump = Bump::new();
    let bytes = Bytes::new(&bump, bump_vec![in &bump; 1, 2, 3]);

    assert_eq!(bytes.into_raw(), bump_vec![in &bump; 1, 2, 3]);
}

#[test]
fn test_set_raw() {
    let bump = Bump::new();
    let mut bytes = Bytes::new(&bump, bump_vec![in &bump; 1, 2, 3]);
    bytes.set_raw(bump_vec![in &bump; 4, 5, 6]);

    assert_eq!(bytes.as_raw(), &bump_vec![in &bump; 4, 5, 6]);
    drop(bytes);
}

#[test]
fn test_push() {
    let bump = Bump::new();
    let mut bytes = Bytes::empty(&bump);
    for i in 0..10 {
        bytes.push(i);
    }
    assert_eq!(
        bytes.as_raw(),
        &bump_vec![in &bump; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    drop(bytes);
}
