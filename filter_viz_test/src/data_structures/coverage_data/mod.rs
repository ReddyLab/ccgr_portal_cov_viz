use std::fmt;

use crate::data_structures::{BucketLoc, ChromosomeData, DbID, Facet, ObservationData};
use rustc_hash::FxHashMap;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

pub mod serialize;

#[derive(Debug)]
pub struct CoverageData {
    pub significant_observations: Vec<ObservationData>,
    pub nonsignificant_observations: Vec<ObservationData>,
    pub bucket_size: u32,
    pub chromosomes: Vec<ChromosomeData>,
    pub facets: Vec<Facet>,
    pub chrom_lengths: Vec<usize>,
    pub features: FxHashMap<DbID, BucketLoc>,
}

const COVERAGE_DATA_FIELD_SIG_OBSERVATIONS: &str = "significant_observations";
const COVERAGE_DATA_FIELD_NONSIG_OBSERVATIONS: &str = "nonsignificant_observations";
const COVERAGE_DATA_FIELD_BUCKET_SIZE: &str = "bucket_size";
const COVERAGE_DATA_FIELD_CHROMOSOMES: &str = "chromosomes";
const COVERAGE_DATA_FIELD_FACETS: &str = "facets";
const COVERAGE_DATA_FIELD_CHROM_LENGTHS: &str = "chrom_lengths";
const COVERAGE_DATA_FIELD_FEATURES: &str = "features";

impl CoverageData {
    pub fn new(
        significant_observations: Vec<ObservationData>,
        nonsignificant_observations: Vec<ObservationData>,
        bucket_size: u32,
        chromosomes: Vec<ChromosomeData>,
        facets: Vec<Facet>,
        chrom_lengths: Vec<usize>,
        features: FxHashMap<DbID, BucketLoc>,
    ) -> Self {
        CoverageData {
            significant_observations,
            nonsignificant_observations,
            bucket_size,
            chromosomes,
            facets,
            chrom_lengths,
            features,
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
        state.serialize_field(COVERAGE_DATA_FIELD_FEATURES, &self.features)?;

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
            Features,
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
                let features = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(CoverageData::new(
                    significant_observations,
                    nonsignificant_observations,
                    bucket_size,
                    chromosomes,
                    facets,
                    chrom_lengths,
                    features,
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
                let mut features = None;
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
                        Field::Features => {
                            if features.is_some() {
                                return Err(de::Error::duplicate_field(
                                    COVERAGE_DATA_FIELD_FEATURES,
                                ));
                            }
                            features = Some(map.next_value()?);
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
                let features = features
                    .ok_or_else(|| de::Error::missing_field(COVERAGE_DATA_FIELD_FEATURES))?;

                Ok(CoverageData::new(
                    significant_observations,
                    nonsignificant_observations,
                    bucket_size,
                    chromosomes,
                    facets,
                    chrom_lengths,
                    features,
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
            COVERAGE_DATA_FIELD_FEATURES,
        ];
        deserializer.deserialize_struct("CoverageData", FIELDS, CoverageDataVisitor)
    }
}
