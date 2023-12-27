use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChromosomeData {
    pub chrom: String,
    pub index: u8,
}

impl ChromosomeData {
    pub fn from(chrom: &str, chrom_idx: u8) -> Self {
        let cd = ChromosomeData {
            chrom: chrom.to_string(),
            index: chrom_idx,
        };
        cd
    }

    // Uncomment if needed. This can be useful for debugging
    // pub fn print_info(&self) {
    //     println!(
    //         "\n{} {} {} {}",
    //         self.chrom,
    //         self.bucket_size,
    //         self.target_intervals.len(),
    //         self.source_intervals.len()
    //     );
    //     print!("    ");
    //     for interval in &self.source_intervals {
    //         let count = interval.values.borrow().facets.len();
    //         if count > 0 {
    //             print!("{} {} ", interval.start, count);
    //         }
    //     }
    //     print!("\n    ");
    //     for interval in &self.target_intervals {
    //         let count = interval.values.borrow().facets.len();
    //         if count > 0 {
    //             print!("{} {} ", interval.start, count);
    //         }
    //     }
    //     println!("");
    // }
}
