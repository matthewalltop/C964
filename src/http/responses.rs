use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlotlyGraph {
    layout: String,
    data: Vec<String>
}

impl PlotlyGraph {
    pub(crate) fn new(layout: String, traces: Vec<String>) -> PlotlyGraph {
        Self {
            layout,
            data: traces
        }
    }
}