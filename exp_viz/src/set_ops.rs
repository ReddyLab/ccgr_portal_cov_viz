use cov_viz_ds::ExperimentFeatureData;
use std::boxed::Box;

#[derive(Clone, Debug)]
pub struct FeatureDataValue {
    pub feature_data: Option<ExperimentFeatureData>,
    pub feature_data_set: Option<Box<SetOp>>,
}

impl FeatureDataValue {
    pub fn data_set(&self) -> Option<ExperimentFeatureData> {
        if self.feature_data.is_some() {
            return self.feature_data.clone();
        } else if let Some(set_op) = &self.feature_data_set {
            return set_op.coalesce();
        }

        return None;
    }
}

#[derive(Clone, Debug)]
pub enum Op {
    Intersection,
    Union,
}

#[derive(Clone, Debug)]
pub struct SetOp {
    pub op: Op,
    pub left: FeatureDataValue,
    pub right: FeatureDataValue,
}

impl SetOp {
    pub fn coalesce(&self) -> Option<ExperimentFeatureData> {
        match (self.left.data_set(), self.right.data_set()) {
            (Some(left), Some(right)) => match self.op {
                Op::Intersection => Some(left.intersection(&right)),
                Op::Union => Some(left.union(&right)),
            },
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        }
    }
}
