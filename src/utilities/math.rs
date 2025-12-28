use rand::distr::{Distribution, uniform::SampleUniform, weighted::WeightedIndex};

use crate::entities::Entity;

// ------------ RNG Math: --------------
pub fn rng_from_range<T>(range_tuple: (T, T)) -> T
where
    T: SampleUniform + PartialOrd + Copy,
{
    rand::random_range(range_tuple.0..=range_tuple.1)
}

pub fn bool_from_chance(chance: f64) -> bool {
    rand::random_bool(chance)
}

pub fn weigh_vec<T>(vec: Vec<(T, f64)>) -> Option<T> {
    if vec.is_empty() {
        return None;
    }

    let (mut items, weights): (Vec<_>, Vec<_>) = vec.into_iter().unzip();

    let mut rng = rand::rng();
    let index_yield = WeightedIndex::new(weights).ok()?.sample(&mut rng);

    Some(items.swap_remove(index_yield))
}

// ------------ Leveling Math:-----------

const A: f64 = 12.677;
const B: f64 = 1218.390;
const C: f64 = -89.075;

pub fn calc_level(xp: f64) -> u16 {
    (((A * (xp + B).ln() + C).floor()) as u16).clamp(1, u16::MAX)
}

pub fn xp_needed(lvl: u16) -> f64 {
    (((lvl as f64 - C) / A).exp() - B).ceil()
}

pub fn calc_xp_gain(entity: &Entity) -> f64 {
    let Entity {
        max_health, damage, ..
    } = *entity;
    let (max_hp, min_dmg, max_dmg) = (max_health as f64, damage.0 as f64, damage.1 as f64);

    let dmg_avg = (min_dmg + max_dmg) / 2.;
    let dmg_delta = (max_dmg - min_dmg) / 2.;

    const OMEGA: f64 = 0.3;
    const RHO: f64 = 0.80;
    const XI: f64 = 0.075;

    let score = max_hp * (dmg_avg + OMEGA * dmg_delta);
    (XI * score.powf(RHO)).ceil()
}

// ----------------------------
