use std::fmt::Display;

use olympus::{
    accu::{AccuNAccumulator, Accumulator, UntakeAccumulator},
    morpho::Tree,
    traits::Image,
    Image2d, Point2d,
};

use crate::{is_1h_face, is_1v_face, Lca};

pub fn accumulate_on_contours_points<A, V>(t: &Tree<Image2d<i32>, u8>, _acc: A) -> Vec<V>
where
    A: Accumulator<Input = Point2d, Output = V> + UntakeAccumulator + Clone,
    V: Display,
{
    let lca = Lca::new(&t);
    let mut contours_acc = vec![A::new(); t.num_nodes()];
    let mut frontiers_acc: Vec<AccuNAccumulator<2, A>> =
        vec![AccuNAccumulator::<2, A>::new(); t.num_nodes()];
    for p in t.nodemap().domain() {
        let a;
        let b;
        if is_1h_face(&p) {
            a = t.node_at_point(&Point2d::new(p.x() - 1, p.y()));
            b = t.node_at_point(&Point2d::new(p.x() + 1, p.y()));
        } else if is_1v_face(&p) {
            a = t.node_at_point(&Point2d::new(p.x(), p.y() - 1));
            b = t.node_at_point(&Point2d::new(p.x(), p.y() + 1));
        } else {
            continue;
        }
        if a != b {
            let lca_n = lca.find(a, b);
            contours_acc.get_mut(a as usize).unwrap().take(p);
            contours_acc.get_mut(b as usize).unwrap().take(p);
            frontiers_acc.get_mut(lca_n as usize).unwrap().take(p);
        }
    }

    println!(
        "[DEBUG] (Before propagation) Root contours: {} frontier: {}",
        contours_acc.get(0).unwrap().result(),
        frontiers_acc.get(0).unwrap().result()
    );

    for n in 1..t.num_nodes() {
        let cur_acc = contours_acc.get(n as usize).unwrap().clone();
        contours_acc
            .get_mut(*t.parent(n as i32) as usize)
            .unwrap()
            .take_accu(&cur_acc);
        contours_acc
            .get_mut(n)
            .unwrap()
            .untake_accu(frontiers_acc.get(n).unwrap().extract_acc());
    }

    contours_acc.into_iter().map(|acc| acc.result()).collect()
}

#[cfg(test)]
mod tests {
    use olympus::{accu::Accumulator, Image2d, Point2d};

    use crate::accumulate_on_contours_points;

    #[test]
    fn test_contours_length() {
        let img = Image2d::<u8>::new_from_vec(
            5,
            5,
            Vec::from([
                2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 1, 4, 4, 2, 2, 3, 3, 3, 2, 2, 2, 2, 2, 2,
            ]),
        );
        const REF: [usize; 4] = [0, 8, 10, 6];
        let t = olympus::morpho::tos(&img, &Point2d::new(0, 0));
        let attr = accumulate_on_contours_points(&t, olympus::accu::CountAccumulator::new());
        for i in 0..4 {
            assert_eq!(REF[i], *attr.get(i).unwrap());
        }
    }
}
