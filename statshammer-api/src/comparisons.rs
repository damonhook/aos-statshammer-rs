use aos_statshammer::{AverageComparisonResult, Opponent, Unit, UnitComparator};
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
