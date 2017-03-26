use rand::Rng;

pub fn select_uniform<'a, T, R: Rng>(items: &'a [T], r: &mut R) -> &'a T {
    &items[r.gen::<usize>() % items.len()]
}

pub fn select_or_select_uniform<'a, T, R: Rng>(first_probability: f64, first: &'a T, rest: &'a [T], r: &mut R) -> &'a T {
    if rest.is_empty() || r.gen::<f64>() < first_probability {
        first
    } else {
        select_uniform(rest, r)
    }
}
