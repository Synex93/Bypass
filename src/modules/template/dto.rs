use crate::builders::builders::BuilderLanguage;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BuilderInfoResp {
    pub languages: Vec<BuilderLanguage>,
}
