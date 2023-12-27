use serde::{Deserialize, Serialize};

use crate::data_structures::DbID;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct BucketLoc {
    pub chrom: u8, // Chromosome index, e.g. chr1 has index 0, chrX is index 22
    pub idx: u32,  // intra-chromosome bucket index
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObservationData {
    pub reo_id: DbID,
    // facet_value_ids should contain the IDs of both the REO's and the associated feature's
    // categorical facet values
    pub facet_value_ids: Vec<DbID>,
    pub source_id: DbID,
    pub target_id: Option<DbID>,
    pub effect_size: f32,
    pub significance: f64,
    pub neg_log_significance: f64,
}
