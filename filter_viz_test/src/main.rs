mod build_data;
mod data_structures;
mod filter;
mod filter_data_structures;
mod load;
mod options;

use std::time::Instant;

use postgres::{Client, NoTls};
use rustc_hash::FxHashSet;
use std::mem;

use filter::filter_coverage_data;
use filter_data_structures::*;
use load::load_coverage_data;

use crate::build_data::build_data;
use crate::options::Options;

fn main() -> () {
    let options = Options::get();

    let data = if options.build {
        let conn_str = options.connection_string.clone().unwrap();
        let mut client = match Client::connect(&conn_str, NoTls) {
            Ok(client) => client,
            Err(e) => {
                panic!("{}", e)
            }
        };

        let data = build_data(&options, &mut client);
        match &data {
            Ok(data) => {
                println!("sig ob count: {}", data.significant_observations.len());
                println!(
                    "nonsig ob count: {}",
                    data.nonsignificant_observations.len()
                );
                data.serialize(&options.output_location)
            }
            Err(e) => eprintln!("{}", e),
        };

        data.unwrap()
    } else {
        let now = Instant::now();
        let data = load_coverage_data(&options.output_location).unwrap();
        println!("Time to decode: {}ms", now.elapsed().as_millis());

        data
    };

    let filters = Filter {
        categorical_facets: FxHashSet::from_iter([4, 5, 6].into_iter()),
        // categorical_facets: FxHashSet::from_iter([].into_iter()),
        numeric_intervals: Some(FilterIntervals {
            effect: (-0.1, 0.1),
            sig: (1.3, 100.0),
        }),
        // numeric_intervals: None,
    };

    let now = Instant::now();
    let filtered_data = filter_coverage_data(&filters, &data);
    println!("Time to filter data: {}ns", now.elapsed().as_nanos());
    println!(
        "{:?}",
        [
            filtered_data.reo_count,
            filtered_data.sources.len(),
            filtered_data.targets.len()
        ]
    );
    mem::forget(data);
    mem::forget(filters);
}
