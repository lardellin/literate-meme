fn main() {
    let response = reqwest::blocking::get("https://data.geo.admin.ch/ch.meteoschweiz.messwerte-lufttemperatur-10min/ch.meteoschweiz.messwerte-lufttemperatur-10min_en.csv");

    let meteo_raw_csv = response.unwrap().text().unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(meteo_raw_csv.as_bytes());

    for row in reader.records() {
        let record = match row {
            Ok(_record) => _record,
            Err(_error) => {
                break;
            }
        };
        // let record = row.unwrap();

        println!("{:?}", record);
    }

    // println!("{meteo_raw_csv}")
}
