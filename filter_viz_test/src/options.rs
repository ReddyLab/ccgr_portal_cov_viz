use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

#[derive(Debug)]
pub struct Options {
    pub build: bool,
    pub output_location: PathBuf,
    pub experiment_accession_id: Option<String>,
    pub assembly_name: Option<String>,
    pub connection_string: Option<String>,
    pub bucket_size: Option<u32>,
    pub chromo: Option<String>,
}

impl Options {
    pub fn get() -> Self {
        let env_args: HashMap<String, String> = env::vars().collect();
        let args: Vec<String> = env::args().collect();

        let build = &args[1] == "--build";

        if !build {
            return Options {
                build: build,
                output_location: PathBuf::from(&args[1]),
                experiment_accession_id: None,
                assembly_name: None,
                bucket_size: None,
                chromo: None,
                connection_string: None,
            };
        }
        let output_location = &args[2];

        let chromo = match args.get(6) {
            Some(chrom_name) => Some(chrom_name.to_string()),
            None => None,
        };

        let output_file = match &chromo {
            Some(chrom_name) => format!("level2_{}.bin", chrom_name.strip_prefix("chr").unwrap()),
            None => "level1.bin".to_string(),
        };
        let path: PathBuf = [output_location, &output_file].iter().collect();

        Options {
            build: build,
            output_location: path,
            experiment_accession_id: Some(args[3].clone()),
            assembly_name: Some(args[4].clone()),
            bucket_size: Some(match args.get(5) {
                Some(size) => u32::from_str_radix(size, 10).unwrap(),
                None => 2_000_000,
            }),
            chromo: chromo,
            connection_string: Some(env_args.get(DATABASE_URL_KEY).unwrap().to_string()),
        }
    }
}
