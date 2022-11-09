use serde::Serialize;

#[derive(Serialize)]
pub(super) struct HealthEndpointResponseModel {
    ok: bool,
}

impl HealthEndpointResponseModel {
    pub(super) fn new() -> Self {
        Self { ok: true }
    }
}
