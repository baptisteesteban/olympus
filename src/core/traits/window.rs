use super::Domain;

pub trait Window {
    type Domain: Domain;

    fn apply(&self, p: &<Self::Domain as Domain>::Point) -> Vec<<Self::Domain as Domain>::Point>;
}
