//! Command line tool to parse and analyze `RINEX` files.    
//! Refer to README for command line arguments.    
//! Based on crate <https://github.com/gwbres/rinex>     
//! Homepage: <https://github.com/gwbres/rinex-cli>
use std::str::FromStr;

use clap::App;
use clap::load_yaml;
use itertools::Itertools;

use rinex::Rinex;
use rinex::Type;
use rinex::record::Sv;
use rinex::constellation::Constellation;

pub fn main () {
	let yaml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yaml);
    app.print_help();
    println!("\n");

	let matches = app.get_matches();
    
    // `Epoch` filters
    let epoch_filter = matches.is_present("epoch");
    let epoch_ok_filter = matches.is_present("epoch-ok");
    
    // `Sv` filters
    let sv = matches.value_of("sv");
    let sv_filters : Option<Vec<Sv>> = match matches.value_of("sv") {
        Some(s) => {
            let sv: Vec<&str> = s.split(",").collect();
            let mut sv_filters : Vec<Sv> = Vec::new();
            for s in sv {
                let constell = Constellation::from_str(&s[0..0])
                    .unwrap();
                let prn = u8::from_str_radix(&s[1..], 10)
                    .unwrap();
                sv_filters.push(Sv::new(constell,prn))
            }
            Some(sv_filters)
        },
        _ => None,
    };

    // OBS data
    let obs_filter = matches.is_present("observation");
    let obs_code_filters = matches.value_of("observation");
    
    // NAV data

    // grab desired RINEX files
    let file_paths : Vec<&str> = matches.value_of("filepath")
        .unwrap()
            .split(",")
            .collect();
    
for fp in &file_paths {
    let p = std::path::PathBuf::from(fp);
    let rinex = match p.exists() {
        true => {
            if let Ok(r) = Rinex::from_file(&std::path::PathBuf::from(fp)) {
                println!("Parsed {:?} RINEX \"{}\"", r.header.rinex_type, fp); 
                r
            } else {
                println!("Failed to parse file \"{}\"", fp); 
                continue
            }
        },
        false => {
            println!("File \"{}\" does not exist", fp);
            continue
        },
    };

    // epoch filter, sort, displayer
    if epoch_filter {
        let record = rinex.record.as_ref()
            .unwrap();
        let e : Vec<_> = match rinex.header.rinex_type {
            Type::ObservationData => {
                record.as_obs().unwrap()
                    .keys()
                    .map(|e| e.date)
                    .sorted()
                    .collect()
            },
            Type::NavigationMessage => {
                record.as_nav().unwrap()
                    .keys()
                    .map(|e| e.date)
                    .sorted()
                    .collect()
            },
            Type::MeteorologicalData => {
                record.as_meteo().unwrap()
                    .keys()
                    .map(|e| e.date)
                    .sorted()
                    .collect()
            }
        };
        println!("*******************************");
        println!("SORTED EPOCHS for \"{}\"\n{:#?}", fp, e);
        println!("*******************************");
    } // epoch filter
/*
    // OBS data filter and manipulation
    if obs {
        let record = rinex.record.as_ref()
            .unwrap();
        let e : Vec<_> = match rinex.header.rinex_type {
            Type::ObservationData => {
                record.as_obs().unwrap()
                .keys()
                    .map(|epoch| {
                        
                    })
                    .sorted()
                    .collect()
            },
            Type::MeteorologicalData => {
            },
        };
        println!("*******************************");
        println!("OBS DATA for \"{}\"\n{:#?}", file_paths[i], e);
        println!("*******************************");
    } // OBS flag
*/
}// for all files
}// main
