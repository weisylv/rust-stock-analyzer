use ingestion::load_demo_data;

fn main() {
    let data = load_demo_data("../../data/sample_aapl.csv").unwrap();
    println!("Loaded {} records", data.len());
    println!("{:#?}", &data[..3]); // show first 3 rows
}