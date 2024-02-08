use crate::application::state::AppState;

#[derive(Debug, Clone)]
pub struct S3State {}

impl S3State {
    pub fn from_state(app: &AppState) -> Self {
        Self {}
    }
}
