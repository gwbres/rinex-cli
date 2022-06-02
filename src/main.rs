//! Command line tool to parse and analyze `RINEX` files.    
//! Refer to README for command line arguments.    
//! Based on crate <https://github.com/gwbres/rinex>     
//! Homepage: <https://github.com/gwbres/rinex-cli>
use clap::App;
use clap::load_yaml;
use std::str::FromStr;

use rinex::Rinex;
use rinex::sv::Sv;
use rinex::epoch;
use rinex::meteo;
use rinex::types::Type;
use rinex::observation;
use rinex::navigation;
use rinex::record::Record;
use rinex::constellation::Constellation;

pub fn main () {
	let yaml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yaml);
    let _ = app.print_help();
    println!("\n");

	let matches = app.get_matches();

    // General
    let resampling = matches.is_present("resampling");
    let decimate = matches.is_present("decimate");
    let merge = matches.is_present("merge");
    let split = matches.is_present("split");
    let splice = matches.is_present("splice");
    let print_record = matches.is_present("record");
    
    // `Epoch`
    let epoch_display = matches.is_present("epoch");
    let epoch_ok_filter = matches.is_present("epoch-ok");
    
    // `Sv`
    let sv_filter : Option<Vec<Sv>> = match matches.value_of("sv") {
        Some(s) => {
            let sv: Vec<&str> = s.split(",").collect();
            let mut sv_filters : Vec<Sv> = Vec::new();
            for s in sv {
                let constell = Constellation::from_str(&s[0..1])
                    .unwrap();
                let prn = u8::from_str_radix(&s[1..], 10)
                    .unwrap();
                sv_filters.push(Sv::new(constell,prn))
            }
            Some(sv_filters)
        },
        _ => None,
    };

    // OBS | METEO
    let obscode_filter : Option<Vec<&str>> = match matches.value_of("codes") {
        Some(s) => Some(s.split(",").collect()),
        _ => None,
    };

    let obscode_display = matches.is_present("obscodes");

    // file paths 
    let filepaths : Vec<&str> = matches.value_of("filepath")
        .unwrap()
            .split(",")
            .collect();

for fp in &filepaths {
    let path = std::path::PathBuf::from(fp);
    let mut rinex = match path.exists() {
        true => {
            if let Ok(r) = Rinex::from_file(fp) {
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

    // [1] resampling
    if resampling {
       println!("resampling is WIP"); 
        let hms = matches.value_of("resampling").unwrap();
        let hms : Vec<_> = hms.split(":").collect();
        let (h,m,s) = (
            u64::from_str_radix(hms[0], 10).unwrap(),
            u64::from_str_radix(hms[1], 10).unwrap(),
            u64::from_str_radix(hms[2], 10).unwrap(),
        );
        let interval = std::time::Duration::from_secs(h*3600 + m*60 +s);
        rinex.resample(interval)
    }
    if decimate {
       println!("resampling is WIP"); 
        let r = u32::from_str_radix(matches.value_of("decimate").unwrap(), 10).unwrap();
        rinex.decimate(r)
    }

    // [2] epoch::ok filter
    if epoch_ok_filter {
        rinex.cleanup()
    }

    // [3] sv filter
    if let Some(ref filter) = sv_filter {
        match &rinex.header.rinex_type {
            Type::ObservationData => {
                let filtered : Vec<_> = rinex.record
                    .as_obs()
                    .unwrap()
                    .iter()
                    .map(|(_e, (_ck, sv))| {
                        sv.iter() 
                            .find(|(v, _)| {
                                 filter.contains(v)
                            })
                    })
                    .collect();
                let mut result = observation::Record::new();
                /*for (e, data) in filtered {
                    result.insert(e, data);
                } */       
                rinex.record = Record::ObsRecord(result)
            },
            _ => {},
        }
    }
        
    if split {
       println!("split is WIP"); 
        /*let datetime = datetime::from_str("%%-%m-%d-%H:%M:%S").unwrap();
        let e = epoch::Epoch::new(
            datetime,
            epoch::EpochFlag::Ok
        );
        let (r1, r2) = rinex.split(e);*/
    }

    if splice {
       println!("splice is WIP"); 
    }
    
    if epoch_display {
        let e : Vec<_> = match rinex.header.rinex_type {
            Type::ObservationData => {
                rinex.record.as_obs().unwrap().keys().collect()
            }
            Type::NavigationData => {
                rinex.record.as_nav().unwrap().keys().collect()
            },
            Type::MeteoData => {
                rinex.record.as_meteo().unwrap().keys().collect()
            },
        };
        println!("*******************************");
        println!("Epochs in \"{}\"\n{:#?}", fp, e);
        println!("*******************************");
    }

    if obscode_display {
        let obs = rinex.header.obs_codes
            .unwrap();
        println!("*******************************");
        println!("OBS in \"{}\"\n{:#?}", fp, obs);
        println!("*******************************");
    }

    if print_record {
        match rinex.header.rinex_type {
            Type::ObservationData => println!("OBS RECORD \n{:#?}", rinex.record.as_obs().unwrap()),
            Type::NavigationData => println!("NAV RECORD \n{:#?}", rinex.record.as_nav().unwrap()),
            Type::MeteoData => println!("METEO RECORD \n{:#?}", rinex.record.as_meteo().unwrap()),
        }
    }
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
