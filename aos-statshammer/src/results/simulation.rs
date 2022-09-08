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

// TODO Compare Speed
// impl From<&HashMap<u32, u32>> for Buckets {
//     fn from(data: &HashMap<u32, u32>) -> Self {
//         let mut discrete: Vec<Bucket> = vec![];
//         let mut cumulative: Vec<Bucket> = vec![];

//         let mut cumulative_total = 0;
//         for (damage, value) in data.iter().sorted() {
//             cumulative_total += value * damage;
//             discrete.push(Bucket {
//                 damage: *damage,
//                 value: *value,
//             });
//             cumulative.push(Bucket {
//                 damage: *damage,
//                 value: cumulative_total,
//             });
//         }

//         Self {
//             discrete,
//             cumulative,
//         }
//     }
// }

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
pub struct SaveSimulatedResult {
    pub save: u32,
    pub buckets: Buckets,
    pub average: f32,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SimulatedUnitResult {
    pub name: String,
    pub results: Vec<SaveSimulatedResult>,
    pub max: u32,
}
