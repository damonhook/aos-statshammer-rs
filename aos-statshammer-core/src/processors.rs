mod roll_target;

mod results;
pub use results::{ProcessorResults, SaveResult};

mod average;
pub use average::AverageDamageProcessor;

mod max;
pub use max::MaxDamageProcessor;
