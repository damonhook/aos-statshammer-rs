use aos_statshammer::{
    average::AverageComparisonResult, simulation::SimulatedUnitResult, Opponent, Unit,
    UnitComparator,
};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AverageComparisonRequest {
    units: Vec<Unit>,
    opponent: Option<Opponent>,
}

#[derive(Serialize)]
pub struct AverageComparisonResponse {
    results: Vec<AverageComparisonResult>,
}

pub async fn compare_average(
    Json(payload): Json<AverageComparisonRequest>,
) -> Json<AverageComparisonResponse> {
    let comparator = UnitComparator::new(&payload.units, payload.opponent.as_ref());
    Json(AverageComparisonResponse {
        results: comparator.compare_average_damage(),
    })
}

#[derive(Deserialize)]
pub struct SimulatedComparisonRequest {
    units: Vec<Unit>,
    opponent: Option<Opponent>,
}

#[derive(Serialize)]
pub struct SimulatedComparisonResponse {
    results: Vec<SimulatedUnitResult>,
}

pub async fn compare_simulated(
    Json(payload): Json<SimulatedComparisonRequest>,
) -> Json<SimulatedComparisonResponse> {
    let comparator = UnitComparator::new(&payload.units, payload.opponent.as_ref());
    Json(SimulatedComparisonResponse {
        results: comparator.compare_simulated_damage(10_000),
    })
}
