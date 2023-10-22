use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Pagination {
    has_more: bool,
    max_per_page: u32,
    next_offset: String,
    results: u32,
}

#[derive(Debug, Deserialize)]
struct ResultItem {
    creation_date: u64,
    id: String,
}

#[derive(Debug, Deserialize)]
struct Data {
    pagination: Pagination,
    results: Vec<ResultItem>,
}
