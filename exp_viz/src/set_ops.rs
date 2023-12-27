use cov_viz_ds::ExperimentFeatureData;
use std::boxed::Box;

pub struct FeatureDataValue {
    pub feature_data: Option<ExperimentFeatureData>,
    pub feature_data_set: Option<Box<SetOp>>,
}

impl FeatureDataValue {
    pub fn data_set(self) -> Option<ExperimentFeatureData> {
        if let Some(feature_data) = self.feature_data {
            return Some(feature_data.to_owned());
        } else if let Some(set_op) = self.feature_data_set {
            return set_op.merge();
        }

        return None;
    }
}

pub enum Op {
    Intersection,
    Union,
}

pub struct SetOp {
    pub op: Op,
    pub left: FeatureDataValue,
    pub right: FeatureDataValue,
}

impl SetOp {
    pub fn merge(self) -> Option<ExperimentFeatureData> {
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
