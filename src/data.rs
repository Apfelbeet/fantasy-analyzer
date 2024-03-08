use std::{path::Path, str::FromStr};

pub fn driver_points() -> Vec<[isize; 20]> {
    read_file("data/drivers_points.csv")
}

pub fn constructor_points() -> Vec<[isize; 10]> {
    read_file("data/constr_points.csv")
}

pub fn driver_costs() -> Vec<[f32; 20]> {
    read_file("data/drivers_cost.csv")
}

pub fn constructor_costs() -> Vec<[f32; 10]> {
    read_file("data/constr_cost.csv")
}

fn read_file<const N: usize, T, P>(file: P) -> Vec<[T; N]>
where
    T: FromStr + Default + Copy,
    <T as FromStr>::Err: std::fmt::Debug,
    P: AsRef<Path>,
{
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file)
        .expect("Couldn't open data file");

    reader
        .records()
        .map(|record| {
            let r = record.unwrap();

            let mut array = [T::default(); N];
            for (driver, value) in r.iter().enumerate() {
                array[driver] = value.parse::<T>().unwrap();
            }
            array
        })
        .collect()
}
