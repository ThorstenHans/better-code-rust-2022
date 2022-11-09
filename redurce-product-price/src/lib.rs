mod models;

use anyhow::{bail, Result};
use models::{ReductionModel, VerifiedReductionModel};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn reduce_product_price(req: Request) -> Result<Response> {
    if req.method() != http::Method::POST {
        bail!("Only POST requests are supported");
    }

    let reduction = parse_payload(&req)?;
    let verified = apply_business_logic(&reduction);

    let payload = serde_json::to_string(&verified)?;

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(payload.into()))?)
}

fn parse_payload(req: &Request) -> Result<ReductionModel> {
    let body = req.body().clone().unwrap_or_default();
    let payload = serde_json::from_slice(&body)?;
    Ok(payload)
}

fn apply_business_logic(rm: &ReductionModel) -> VerifiedReductionModel {
    let verified = rm.reduce_by < 0.3 && rm.reduce_by > 0.0;

    VerifiedReductionModel::new(rm.id, rm.reduce_by, verified)
}
