use util::*;

#[test]
fn bidirectional_list_get_mut_with_default() {
    let mut l = BidirectionalList::new();

    assert_eq!(l.get(0), None);

    *l.get_mut_with_default(-1) = 1;
    *l.get_mut_with_default(1) = 2;

    assert_eq!(l.get_checked(0), &0);
    assert_eq!(l.get_checked(-1), &1);
    assert_eq!(l.get_checked(1), &2);
}
