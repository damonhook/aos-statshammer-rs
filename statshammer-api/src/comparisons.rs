use crate::errors::ApiError;
use aos_statshammer::{
    average::AverageComparisonResult, simulation::SimulatedUnitResult, Opponent, Unit,
    UnitComparator,
};
use axum::{extract::Query, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AverageComparisonRequestBody {
    units: Vec<Unit>,
    opponent: Option<Opponent>,
}

#[derive(Serialize)]
pub struct AverageComparisonResponse {
    results: Vec<AverageComparisonResult>,
}

pub async fn compare_average(
    Json(payload): Json<AverageComparisonRequestBody>,
) -> Json<AverageComparisonResponse> {
    let comparator = UnitComparator::new(&payload.units, payload.opponent.as_ref());
    Json(AverageComparisonResponse {
        results: comparator.compare_average_damage(),
    })
}

static DEFAULT_ITERATIONS: u32 = 5_000;
static MAX_ITERATIONS: u32 = 50_000;

#[derive(Deserialize)]
pub struct SimulatedComparisonRequestQuery {
    save: u32,
    #[serde(default = "default_iterations")]
    iterations: u32,
}

fn default_iterations() -> u32 {
    DEFAULT_ITERATIONS
}

#[derive(Deserialize)]
pub struct SimulatedComparisonRequestBody {
    units: Vec<Unit>,
    opponent: Option<Opponent>,
}

#[derive(Serialize)]
pub struct SimulatedComparisonResponse {
    results: Vec<SimulatedUnitResult>,
}

pub async fn compare_simulated(
    Query(query): Query<SimulatedComparisonRequestQuery>,
    Json(payload): Json<SimulatedComparisonRequestBody>,
) -> Result<Json<SimulatedComparisonResponse>, ApiError> {
    if query.iterations > MAX_ITERATIONS {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            &format!(
                "Too many iterations. Max: {}, Requested: {}",
                MAX_ITERATIONS, query.iterations
            ),
        ));
    }
    let comparator = UnitComparator::new(&payload.units, payload.opponent.as_ref());
    Ok(Json(SimulatedComparisonResponse {
        results: comparator.compare_simulated_damage(query.save, query.iterations),
    }))
}
