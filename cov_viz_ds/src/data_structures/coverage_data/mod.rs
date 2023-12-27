use std::fmt;

use crate::data_structures::{BucketLoc, ChromosomeData, DbID, Facet, ObservationData};
use roaring::RoaringTreemap;
use rustc_hash::FxHashMap;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

pub mod serialize;

#[derive(Clone, Debug)]
pub struct CoverageData {
    pub significant_observations: Vec<ObservationData>,
    pub nonsignificant_observations: Vec<ObservationData>,
    pub bucket_size: u32,
    pub chromosomes: Vec<ChromosomeData>,
    pub facets: Vec<Facet>,
    pub chrom_lengths: Vec<usize>,
    pub feature_buckets: FxHashMap<DbID, BucketLoc>,
}

const COVERAGE_DATA_FIELD_SIG_OBSERVATIONS: &str = "significant_observations";
const COVERAGE_DATA_FIELD_NONSIG_OBSERVATIONS: &str = "nonsignificant_observations";
const COVERAGE_DATA_FIELD_BUCKET_SIZE: &str = "bucket_size";
const COVERAGE_DATA_FIELD_CHROMOSOMES: &str = "chromosomes";
const COVERAGE_DATA_FIELD_FACETS: &str = "facets";
const COVERAGE_DATA_FIELD_CHROM_LENGTHS: &str = "chrom_lengths";
const COVERAGE_DATA_FIELD_FEATURE_BUCKETS: &str = "feature_buckets";

impl CoverageData {
    pub fn new(
        significant_observations: Vec<ObservationData>,
        nonsignificant_observations: Vec<ObservationData>,
        bucket_size: u32,
        chromosomes: Vec<ChromosomeData>,
        facets: Vec<Facet>,
        chrom_lengths: Vec<usize>,
        feature_buckets: FxHashMap<DbID, BucketLoc>,
    ) -> Self {
        CoverageData {
            significant_observations,
            nonsignificant_observations,
            bucket_size,
            chromosomes,
            facets,
            chrom_lengths,
            feature_buckets,
        }
    }

    // Uncomment if needed. This can be useful for debugging
    // pub fn print_chroms(&self) {
    //     for chrom in &self.chromosomes {
    //         chrom.print_info();
    //     }
    // }
}

impl Serialize for CoverageData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CoverageData", 2)?;
        state.serialize_field(
            COVERAGE_DATA_FIELD_SIG_OBSERVATIONS,
            &self.significant_observations,
        )?;
        state.serialize_field(
            COVERAGE_DATA_FIELD_NONSIG_OBSERVATIONS,
            &self.nonsignificant_observations,
        )?;
        state.serialize_field(COVERAGE_DATA_FIELD_BUCKET_SIZE, &self.bucket_size)?;
        state.serialize_field(COVERAGE_DATA_FIELD_CHROMOSOMES, &self.chromosomes)?;
        state.serialize_field(COVERAGE_DATA_FIELD_FACETS, &self.facets)?;
        state.serialize_field(COVERAGE_DATA_FIELD_CHROM_LENGTHS, &self.chrom_lengths)?;
        state.serialize_field(COVERAGE_DATA_FIELD_FEATURE_BUCKETS, &self.feature_buckets)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for CoverageData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Significant_Observations,
            Nonsignificant_Observations,
            Bucket_Size,
            Chromosomes,
            Facets,
            Chrom_Lengths,
            Feature_Buckets,
        }

        struct CoverageDataVisitor;

        impl<'de> Visitor<'de> for CoverageDataVisitor {
            type Value = CoverageData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CoverageData")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<CoverageData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let significant_observations = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let nonsignificant_observations = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let bucket_size = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let chromosomes = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let facets = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let chrom_lengths = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let feature_buckets = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(CoverageData::new(
                    significant_observations,
                    nonsignificant_observations,
                    bucket_size,
                    chromosomes,
                    facets,
                    chrom_lengths,
                    feature_buckets,
                ))
            }

            fn visit_map<V>(self, mut map: V) -> Result<CoverageData, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut significant_observations = None;
                let mut nonsignificant_observations = None;
                let mut bucket_size = None;
                let mut chromosomes = None;
                let mut facets = None;
                let mut chrom_lengths = None;
                let mut feature_buckets = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Significant_Observations => {
                            if significant_observations.is_some() {
                                return Err(de::Error::duplicate_field(
                                    COVERAGE_DATA_FIELD_SIG_OBSERVATIONS,
                                ));
                            }
                            significant_observations = Some(map.next_value()?);
                        }
                        Field::Nonsignificant_Observations => {
                            if nonsignificant_observations.is_some() {
                                return Err(de::Error::duplicate_field(
                                    COVERAGE_DATA_FIELD_NONSIG_OBSERVATIONS,
                                ));
                            }
                            nonsignificant_observations = Some(map.next_value()?);
                        }
                        Field::Bucket_Size => {
                            if bucket_size.is_some() {
                                return Err(de::Error::duplicate_field(
                                    COVERAGE_DATA_FIELD_BUCKET_SIZE,
                                ));
                            }
                            bucket_size = Some(map.next_value()?);
                        }
                        Field::Chromosomes => {
                            if chromosomes.is_some() {
                                return Err(de::Error::duplicate_field(
                                    COVERAGE_DATA_FIELD_CHROMOSOMES,
                                ));
                            }
                            chromosomes = Some(map.next_value()?);
                        }
                        Field::Facets => {
                            if facets.is_some() {
                                return Err(de::Error::duplicate_field(COVERAGE_DATA_FIELD_FACETS));
                            }
                            facets = Some(map.next_value()?);
                        }
                        Field::Chrom_Lengths => {
                            if chrom_lengths.is_some() {
                                return Err(de::Error::duplicate_field(
                                    COVERAGE_DATA_FIELD_CHROM_LENGTHS,
                                ));
                            }
                            chrom_lengths = Some(map.next_value()?);
                        }
                        Field::Feature_Buckets => {
                            if feature_buckets.is_some() {
                                return Err(de::Error::duplicate_field(
                                    COVERAGE_DATA_FIELD_FEATURE_BUCKETS,
                                ));
                            }
                            feature_buckets = Some(map.next_value()?);
                        }
                    }
                }
                let significant_observations = significant_observations.ok_or_else(|| {
                    de::Error::missing_field(COVERAGE_DATA_FIELD_SIG_OBSERVATIONS)
                })?;
                let nonsignificant_observations = nonsignificant_observations.ok_or_else(|| {
                    de::Error::missing_field(COVERAGE_DATA_FIELD_NONSIG_OBSERVATIONS)
                })?;
                let bucket_size = bucket_size
                    .ok_or_else(|| de::Error::missing_field(COVERAGE_DATA_FIELD_BUCKET_SIZE))?;
                let chromosomes = chromosomes
                    .ok_or_else(|| de::Error::missing_field(COVERAGE_DATA_FIELD_CHROMOSOMES))?;
                let facets =
                    facets.ok_or_else(|| de::Error::missing_field(COVERAGE_DATA_FIELD_FACETS))?;
                let chrom_lengths = chrom_lengths
                    .ok_or_else(|| de::Error::missing_field(COVERAGE_DATA_FIELD_CHROM_LENGTHS))?;
                let feature_buckets = feature_buckets
                    .ok_or_else(|| de::Error::missing_field(COVERAGE_DATA_FIELD_FEATURE_BUCKETS))?;

                Ok(CoverageData::new(
                    significant_observations,
                    nonsignificant_observations,
                    bucket_size,
                    chromosomes,
                    facets,
                    chrom_lengths,
                    feature_buckets,
                ))
            }
        }

        const FIELDS: &'static [&'static str] = &[
            COVERAGE_DATA_FIELD_SIG_OBSERVATIONS,
            COVERAGE_DATA_FIELD_NONSIG_OBSERVATIONS,
            COVERAGE_DATA_FIELD_BUCKET_SIZE,
            COVERAGE_DATA_FIELD_CHROMOSOMES,
            COVERAGE_DATA_FIELD_FACETS,
            COVERAGE_DATA_FIELD_CHROM_LENGTHS,
            COVERAGE_DATA_FIELD_FEATURE_BUCKETS,
        ];
        deserializer.deserialize_struct("CoverageData", FIELDS, CoverageDataVisitor)
    }
}

#[derive(Clone, Debug)]
pub struct ExperimentFeatureData {
    pub sources: RoaringTreemap,
    pub targets: RoaringTreemap,
}

const EXPERIMENT_FEATURE_DATA_FIELD_SOURCES: &str = "sources";
const EXPERIMENT_FEATURE_DATA_FIELD_TARGETS: &str = "targets";

impl ExperimentFeatureData {
    pub fn new(sources: RoaringTreemap, targets: RoaringTreemap) -> Self {
        ExperimentFeatureData { sources, targets }
    }

    pub fn default() -> Self {
        ExperimentFeatureData {
            sources: RoaringTreemap::default(),
            targets: RoaringTreemap::default(),
        }
    }

    // Uncomment if needed. This can be useful for debugging
    // pub fn print_chroms(&self) {
    //     for chrom in &self.chromosomes {
    //         chrom.print_info();
    //     }
    // }
}

impl Serialize for ExperimentFeatureData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ExperimentFeatureData", 2)?;

        let mut source_data = vec![];
        let _ = self.sources.serialize_into(&mut source_data);
        state.serialize_field(EXPERIMENT_FEATURE_DATA_FIELD_SOURCES, &source_data)?;
        let mut target_data = vec![];
        let _ = self.targets.serialize_into(&mut target_data);
        state.serialize_field(EXPERIMENT_FEATURE_DATA_FIELD_TARGETS, &target_data)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for ExperimentFeatureData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Sources,
            Targets,
        }

        struct ExperimentFeatureDataVisitor;

        impl<'de> Visitor<'de> for ExperimentFeatureDataVisitor {
            type Value = ExperimentFeatureData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ExperimentFeatureData")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<ExperimentFeatureData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let source_data: Vec<u8> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let target_data: Vec<u8> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let sources = RoaringTreemap::deserialize_from(&source_data[..]).unwrap();
                let targets = RoaringTreemap::deserialize_from(&target_data[..]).unwrap();

                Ok(ExperimentFeatureData::new(sources, targets))
            }

            fn visit_map<V>(self, mut map: V) -> Result<ExperimentFeatureData, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut source_data: Option<Vec<u8>> = None;
                let mut target_data: Option<Vec<u8>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Sources => {
                            if source_data.is_some() {
                                return Err(de::Error::duplicate_field(
                                    EXPERIMENT_FEATURE_DATA_FIELD_SOURCES,
                                ));
                            }
                            source_data = Some(map.next_value()?);
                        }
                        Field::Targets => {
                            if target_data.is_some() {
                                return Err(de::Error::duplicate_field(
                                    EXPERIMENT_FEATURE_DATA_FIELD_TARGETS,
                                ));
                            }
                            target_data = Some(map.next_value()?);
                        }
                    }
                }

                let source_data = source_data.ok_or_else(|| {
                    de::Error::missing_field(EXPERIMENT_FEATURE_DATA_FIELD_SOURCES)
                })?;
                let target_data = target_data.ok_or_else(|| {
                    de::Error::missing_field(EXPERIMENT_FEATURE_DATA_FIELD_TARGETS)
                })?;
                let sources = RoaringTreemap::deserialize_from(&source_data[..]).unwrap();
                let targets = RoaringTreemap::deserialize_from(&target_data[..]).unwrap();

                Ok(ExperimentFeatureData::new(sources, targets))
            }
        }

        const FIELDS: &'static [&'static str] = &[
            EXPERIMENT_FEATURE_DATA_FIELD_SOURCES,
            EXPERIMENT_FEATURE_DATA_FIELD_TARGETS,
        ];
        deserializer.deserialize_struct(
            "ExperimentFeatureData",
            FIELDS,
            ExperimentFeatureDataVisitor,
        )
    }
}
