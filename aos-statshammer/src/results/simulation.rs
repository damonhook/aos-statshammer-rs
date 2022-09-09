use itertools::Itertools;
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Bucket {
    pub damage: u32,
    pub value: u32,
}

impl Bucket {
    pub fn new(damage: u32, value: u32) -> Self {
        Self { damage, value }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Buckets {
    pub discrete: Vec<Bucket>,
    pub cumulative: Vec<Bucket>,
}

impl From<&HashMap<u32, u32>> for Buckets {
    fn from(data: &HashMap<u32, u32>) -> Self {
        let mut cumulative_total = 0;
        let (discrete, cumulative): (Vec<_>, Vec<_>) = data
            .iter()
            .sorted()
            .map(|(damage, value)| {
                cumulative_total += value;
                (
                    Bucket::new(*damage, *value),
                    Bucket::new(*damage, cumulative_total),
                )
            })
            .unzip();

        Self {
            discrete,
            cumulative,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SimulatedUnitResult {
    pub name: String,
    pub results: Buckets,
    pub max: u32,
    pub average: f32,
}
