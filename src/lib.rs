pub mod parsing {

    const TIMEZONE: &str = "Europe/Zurich";
    #[derive(Clone, Debug, Copy)]
    pub enum Variable {
        Temperature,
        Humidity,
        Precipitation,
    }

    impl Variable {
        fn map_url(self) -> String {
            return match self {
                Variable::Temperature => "lufttemperatur".to_string(),
                Variable::Humidity => "luftfeuchtigkeit".to_string(),
                Variable::Precipitation => "niederschlag".to_string(),
            };
        }

        fn name(self) -> String {
            return match self {
                Variable::Temperature => "temperature".to_string(),
                Variable::Humidity => "humidity".to_string(),
                Variable::Precipitation => "precipitations".to_string(),
            };
        }
    }

    pub mod measurements {

        use chrono::{DateTime, NaiveDateTime, Utc};
        use chrono_tz::Tz;
        const TIMEZONE: &str = super::TIMEZONE;
        const CSV_DATEIME_FMT: &str = "%Y-%m-%d %H:%M";

        #[derive(Debug)]
        pub struct Measurement {
            name: String,
            short: String,
            value: f64,
            variable: String,
            datetime: DateTime<Utc>,
        }

        pub fn read_measurements(var: super::Variable) -> Vec<Measurement> {
            let tz: Tz = TIMEZONE.parse().unwrap();

            let response = reqwest::blocking::get(
                format!(
                    concat!(
                        "https://data.geo.admin.ch/",
                        "ch.meteoschweiz.messwerte-{}-10min/",
                        "ch.meteoschweiz.messwerte-{}-10min_en.csv"
                    ),
                    var.map_url(),
                    var.map_url()
                )
                .as_str(),
            );

            let meteo_raw_csv = response.unwrap().text().unwrap();

            // define the reader for meteosuisse CSVs.
            let mut reader = csv::ReaderBuilder::new()
                .delimiter(b';')
                .from_reader(meteo_raw_csv.as_bytes());

            let mut measurements: Vec<Measurement> = Vec::new();

            for row in reader.records() {
                let record = match row {
                    // results are automatically extracted
                    Ok(_record) => _record,
                    // at the first error the cycle is terminated
                    Err(_error) => {
                        break;
                    }
                };

                let measurement = Measurement {
                    name: record.get(0).unwrap().to_string(),
                    short: record.get(1).unwrap().to_string(),
                    value: record
                        .get(3)
                        .unwrap()
                        .parse::<f64>()
                        .expect("string format not compatible to float64"),
                    variable: var.name(),
                    datetime: NaiveDateTime::parse_from_str(
                        record.get(4).unwrap(),
                        CSV_DATEIME_FMT,
                    )
                    .unwrap()
                    .and_local_timezone(tz)
                    .unwrap()
                    .to_utc(),
                };

                // println!("{:?}", measurement);
                measurements.push(measurement);
            }

            return measurements;
        }
    }
}
