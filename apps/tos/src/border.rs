use olympus::{
    traits::{Image, ImageFromDomain},
    Box2d, Image2d,
};

pub fn compute_median_on_border<T>(img: &Image2d<T>) -> T
where
    T: Ord + Copy,
{
    let mut border_values =
        Vec::<T>::with_capacity((2 * (img.width() + img.height()) - 4) as usize);
    for x in 0..img.width() {
        border_values.push(*img.at(x, 0));
        border_values.push(*img.at(x, img.height() - 1));
    }
    for y in 1..(img.height() - 1) {
        border_values.push(*img.at(0, y));
        border_values.push(*img.at(img.width() - 1, y));
    }
    border_values.sort();
    *border_values.get(border_values.len() / 2).unwrap()
}

pub fn add_border<T>(img: &Image2d<T>, v: T) -> Image2d<T>
where
    T: Copy + Default + Ord,
{
    let domain = img.domain();
    let new_domain = Box2d::new_from_dimension(domain.width() + 2, domain.height() + 2);
    let mut res = Image2d::<T>::new_from_domain(&new_domain);

    for x in 0..img.width() {
        for y in 0..img.height() {
            *res.at_mut(x + 1, y + 1) = *img.at(x, y);
        }
    }

    for x in 0..res.width() {
        *res.at_mut(x, 0) = v;
        *res.at_mut(x, res.height() - 1) = v;
    }
    for y in 1..(res.height() - 1) {
        *res.at_mut(0, y) = v;
        *res.at_mut(res.width() - 1, y) = v;
    }

    res
}

pub fn add_median_border<T>(img: &Image2d<T>) -> Image2d<T>
where
    T: Ord + Copy + Default,
{
    let v = compute_median_on_border(img);
    add_border(img, v)
}
