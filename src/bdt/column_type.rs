use serde::{Serialize};

#[derive(Debug, Serialize)]
pub enum ColumnType {
    Date,
    Text,
    Num,
    Cdf {
        codificator_id: String,
        select_params: String,
    },
}
