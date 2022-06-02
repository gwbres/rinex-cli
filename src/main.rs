//! Command line tool to parse and analyze `RINEX` files.    
//! Refer to README for command line arguments.    
//! Based on crate <https://github.com/gwbres/rinex>     
//! Homepage: <https://github.com/gwbres/rinex-cli>
use clap::App;
use clap::load_yaml;
use std::str::FromStr;
use std::collections::HashMap;

use rinex::Rinex;
use rinex::sv::Sv;
use rinex::meteo;
use rinex::types::Type;
use rinex::observation;
use rinex::navigation;
use rinex::record::Record;
use rinex::constellation::Constellation;

pub fn main () {
	let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    //let _ = app.print_help();
    //println!("\n");

	let matches = app.get_matches();

    // General
    let resampling = matches.is_present("resampling");
    let decimate = matches.is_present("decimate");
    let merge = matches.is_present("merge");
    let split = matches.is_present("split");
    let splice = matches.is_present("splice");
    
    // `Epoch`
    let epoch_display = matches.is_present("epoch");
    let epoch_ok_filter = matches.is_present("epoch-ok");
    let epoch_nok_filter = matches.is_present("epoch-nok");
    
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
    let obscode_display = matches.is_present("obscodes");
    let obscode_filter : Option<Vec<&str>> = match matches.value_of("codes") {
        Some(s) => Some(s.split(",").collect()),
        _ => None,
    };

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
        match &rinex.header.rinex_type {
            Type::ObservationData => {
                let filtered : Vec<_> = rinex.record
                    .as_obs()
                    .unwrap()
                    .iter()
                    .filter(|(epoch, (_, _))| {
                        epoch.flag.is_ok()
                    })
                    .collect();
                let mut rework = observation::Record::new();
                for (e, data) in filtered {
                    rework.insert(*e, data.clone());
                }
                rinex.record = Record::ObsRecord(rework)
            },
            Type::MeteoData => {
                let filtered : Vec<_> = rinex.record
                    .as_meteo()
                    .unwrap()
                    .iter()
                    .filter(|(epoch, _)| {
                        epoch.flag.is_ok()
                    })
                    .collect();
                let mut rework = meteo::Record::new();
                for (e, data) in filtered {
                    rework.insert(*e, data.clone());
                }
                rinex.record = Record::MeteoRecord(rework)
            },
            _ => {},
        }
    }
    // [2*] !epoch::ok filter
    if epoch_nok_filter {
        match &rinex.header.rinex_type {
            Type::ObservationData => {
                let filtered : Vec<_> = rinex.record
                    .as_obs()
                    .unwrap()
                    .iter()
                    .filter(|(epoch, (_, _))| {
                        !epoch.flag.is_ok()
                    })
                    .collect();
                let mut rework = observation::Record::new();
                for (e, data) in filtered {
                    rework.insert(*e, data.clone());
                }
                rinex.record = Record::ObsRecord(rework)
            },
            Type::MeteoData => {
                let filtered : Vec<_> = rinex.record
                    .as_meteo()
                    .unwrap()
                    .iter()
                    .filter(|(epoch, _)| {
                        !epoch.flag.is_ok()
                    })
                    .collect();
                let mut rework = meteo::Record::new();
                for (e, data) in filtered {
                    rework.insert(*e, data.clone());
                }
                rinex.record = Record::MeteoRecord(rework)
            },
            _ => {},
        }
    }

    // [3] sv filter
    if let Some(ref filter) = sv_filter {
        match &rinex.header.rinex_type {
            Type::ObservationData => {
                let mut rework = observation::Record::new();
                for (epoch, (ck,data)) in rinex.record.as_obs().unwrap().iter() {
                    let mut map : HashMap<Sv, HashMap<String, observation::ObservationData>> = HashMap::new();
                    for (sv, data) in data.iter() {
                        if filter.contains(sv) {
                            map.insert(*sv, data.clone());
                        }
                    }
                    if map.len() > 0 {
                        rework.insert(*epoch, (*ck, map));
                    }
                }
                rinex.record = Record::ObsRecord(rework)
            },
            Type::NavigationData => {
                let mut rework = navigation::Record::new();
                for (epoch, data) in rinex.record.as_nav().unwrap().iter() {
                    let mut map : HashMap<Sv, HashMap<String, navigation::ComplexEnum>> = HashMap::new();
                    for (sv, data) in data.iter() {
                        if filter.contains(sv) {
                            map.insert(*sv, data.clone());
                        }
                    }
                    if map.len() > 0 {
                        rework.insert(*epoch, map);
                    }
                }
                rinex.record = Record::NavRecord(rework)
            },
            _ => {},
        }
    }

    // [4] OBS code filter
    if let Some(ref filter) = obscode_filter {
        match &rinex.header.rinex_type {
            Type::ObservationData => {
                let mut rework = observation::Record::new();
                for (epoch, (ck,data)) in rinex.record.as_obs().unwrap().iter() {
                    let mut map : HashMap<Sv, HashMap<String, observation::ObservationData>> = HashMap::new();
                    for (sv, data) in data.iter() {
                        let mut inner : HashMap<String, observation::ObservationData> = HashMap::new();
                        for (code, data) in data.iter() {
                            if filter.contains(&code.as_str()) {
                                inner.insert(code.clone(), data.clone());
                            }
                        }
                        if inner.len() > 0 {
                            map.insert(*sv, inner);
                        }
                    }
                    if map.len() > 0 {
                        rework.insert(*epoch, (*ck, map));
                    }
                }
                rinex.record = Record::ObsRecord(rework)
            },
            Type::MeteoData => {
                let mut rework = meteo::Record::new();
                for (epoch, data) in rinex.record.as_meteo().unwrap().iter() {
                    let mut map : HashMap<String, f32> = HashMap::new(); 
                    for (code, data) in data.iter() {
                        if filter.contains(&code.as_str()) {
                            map.insert(code.clone(), *data);
                        }
                    }
                    if map.len() > 0 {
                        rework.insert(*epoch, map);
                    }
                }
                rinex.record = Record::MeteoRecord(rework)
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

    if merge {
        println!("MERGE() is WIP");
    }

    if !epoch_display && !obscode_display {
        match rinex.header.rinex_type {
            Type::ObservationData => println!("OBS RECORD \n{:#?}", rinex.record.as_obs().unwrap()),
            Type::NavigationData => println!("NAV RECORD \n{:#?}", rinex.record.as_nav().unwrap()),
            Type::MeteoData => println!("METEO RECORD \n{:#?}", rinex.record.as_meteo().unwrap()),
        }
    }

}// for all files
}// main
