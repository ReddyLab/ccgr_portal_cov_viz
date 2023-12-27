use std::path::PathBuf;

use crate::data_structures::CoverageData;

/// Loads the coverage data from disk
pub fn load_coverage_data(location: &PathBuf) -> Result<CoverageData, bincode::Error> {
    CoverageData::deserialize(&location)
}
