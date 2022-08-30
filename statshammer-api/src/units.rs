use aos_statshammer::{AverageComparisonResult, Unit, UnitComparator};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AverageComparisonRequest {
    units: Vec<Unit>,
}

#[derive(Serialize)]
pub struct AverageComparisonResponse {
    results: Vec<AverageComparisonResult>,
}

pub async fn compare_average(
    Json(payload): Json<AverageComparisonRequest>,
) -> Json<AverageComparisonResponse> {
    let comparator = UnitComparator::new(&payload.units);
    Json(AverageComparisonResponse {
        results: comparator.compare_average_damage(),
    })
}
