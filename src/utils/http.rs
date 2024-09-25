use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub count: Option<usize>,
    pub data: Option<T>,
    pub error: Option<String>,
}
