use coord::*;

#[test]
fn rect_border() {
    let rect = Rect::new(Coord::new(1, 2), 2, 3).unwrap();

    let mut i = 0;
    for coord in rect.border_iter() {
        assert_eq!(coord, rect.border_get(i).unwrap());
        i += 1;
    }
}
