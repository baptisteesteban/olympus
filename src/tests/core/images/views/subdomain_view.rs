use crate::{traits::Image, views::SubdomainView, Box2d, Image2d};

#[test]
fn test_subdomain_box2d() {
    const REF_VAL: [u8; 4] = [1, 2, 4, 5];
    let img = Image2d::new_from_vec(3, 3, Vec::<u8>::from([1, 2, 3, 4, 5, 6, 7, 8, 9]));
    let new_domain = Box2d::new(2, 2);
    let view = SubdomainView::new(&img, new_domain);
    let mut i = 0;

    for p in view.domain() {
        assert_eq!(*view.at_point(&p), REF_VAL[i]);
        i += 1;
    }
}
