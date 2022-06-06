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
use rinex::navigation;
use rinex::observation;
use rinex::types::Type;
use rinex::epoch;
use rinex::record::Record;
use rinex::constellation::Constellation;

pub fn main () {
	let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
	let matches = app.get_matches();

    // General 
    let pretty = matches.is_present("pretty");
    let filepaths : Vec<&str> = matches.value_of("filepath")
        .unwrap()
            .split(",")
            .collect();
    let output : Option<Vec<&str>> = match matches.is_present("output") {
        true => {
            Some(matches.value_of("output")
                .unwrap()
                    .split(",")
                    .collect())
        },
        false => None,
    };

    // RINEX 
    let header = matches.is_present("header");
    let resampling = matches.is_present("resampling");
    let decimate = matches.is_present("decimate");

    // SPEC ops
    let merge = matches.is_present("merge");

    let split = matches.is_present("split");
    let split_epoch : Option<epoch::Epoch> = match matches.value_of("split") {
        Some(date) => {
            let offset = 4 +2+1 +2+1 +2+1 +2+1 +2+1; // YYYY-mm-dd-HH:MM:SS 
            let datetime = date[0..offset].to_string();
            let flag : Option<epoch::EpochFlag> = match date.len() > offset {
                true => Some(epoch::EpochFlag::from_str(&date[offset+1..])
                    .unwrap_or(epoch::EpochFlag::Ok)),
                false => None,
            };
            Some(epoch::Epoch {
                date : chrono::NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S")
                    .unwrap(), 
                flag : flag.unwrap_or(epoch::EpochFlag::Ok),
            })
        },
        None => None,
    };
    
    let splice = matches.is_present("splice");

    let spec_ops = merge | split | splice;
    
    // `Epoch`
    let epoch_display = matches.is_present("epoch");
    let epoch_ok_filter = matches.is_present("epoch-ok");
    let epoch_nok_filter = matches.is_present("epoch-nok");

    // `GNSS`
    let gnss_filter : Option<Vec<Constellation>> = match matches.value_of("constellation") {
        Some(gnss) => {
            let data : Vec<&str> = gnss.split(",").collect();
            let mut gnss : Vec<Constellation> = Vec::new();
            for d in data {
                gnss.push(Constellation::from_str(d).unwrap())
            }
            Some(gnss)
        },
        _ => None,
    };
    
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

    // Data Filters 
    let obscode_display = matches.is_present("obscodes");
    let obscode_filter : Option<Vec<&str>> = match matches.value_of("codes") {
        Some(s) => Some(s.split(",").collect()),
        _ => None,
    };
    let has_lli = matches.is_present("has-lli");
    let lli : Option<u8> = match matches.value_of("lli") {
        Some(s) => Some(u8::from_str_radix(s,10).unwrap()),
        _ => None,
    };
    let has_ssi = matches.is_present("has-ssi");
    let ssi : Option<observation::Ssi> = match matches.value_of("ssi") {
        Some(s) => Some(observation::Ssi::from_str(s).unwrap()),
        _ => None,
    };

    let mut index : usize = 0;
    let mut merged: Rinex = Rinex::default();
    let mut to_merge : Vec<Rinex> = Vec::new(); 

for fp in &filepaths {
    let path = std::path::PathBuf::from(fp);
    let mut rinex = match path.exists() {
        true => {
            if let Ok(r) = Rinex::from_file(fp) {
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

    if index == 0 {
        merged = rinex.clone()
    } else {
        to_merge.push(rinex.clone())
    }
    index += 1;

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

    // [3] GNSS filter
    if let Some(ref filter) = gnss_filter {
        match &rinex.header.rinex_type {
            Type::ObservationData => {
                let mut rework = observation::Record::new();
                for (epoch, (ck,data)) in rinex.record.as_obs().unwrap().iter() {
                    let mut map : HashMap<Sv, HashMap<String, observation::ObservationData>> = HashMap::new();
                    for (sv, data) in data.iter() {
                        if filter.contains(&sv.constellation) {
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
                        if filter.contains(&sv.constellation) {
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
    // [3*] sv filter
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
            Type::NavigationData => {
                let mut rework = navigation::Record::new();
                for (epoch, data) in rinex.record.as_nav().unwrap().iter() {
                    let mut map : HashMap<Sv, HashMap<String, navigation::ComplexEnum>> = HashMap::new();
                    for (sv, data) in data.iter() {
                        let mut inner : HashMap<String, navigation::ComplexEnum> = HashMap::new();
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
                        rework.insert(*epoch, map);
                    }
                }
                rinex.record = Record::NavRecord(rework)
            },
        }
    }
    //[4*] LLI presence filter
    if has_lli {
        match &rinex.header.rinex_type {
            Type::ObservationData => {
                let mut rework = observation::Record::new();
                for (epoch, (ck,data)) in rinex.record.as_obs().unwrap().iter() {
                    let mut map : HashMap<Sv, HashMap<String, observation::ObservationData>> = HashMap::new();
                    for (sv, data) in data.iter() {
                        let mut inner : HashMap<String, observation::ObservationData> = HashMap::new();
                        for (code, data) in data.iter() {
                            if let Some(lli_flags) = data.lli {
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
            _ => {},
        }
    }
    //[4*] LLI filter
    if let Some(lli) = lli {
        match &rinex.header.rinex_type {
            Type::ObservationData => {
                let mut rework = observation::Record::new();
                for (epoch, (ck,data)) in rinex.record.as_obs().unwrap().iter() {
                    let mut map : HashMap<Sv, HashMap<String, observation::ObservationData>> = HashMap::new();
                    for (sv, data) in data.iter() {
                        let mut inner : HashMap<String, observation::ObservationData> = HashMap::new();
                        for (code, data) in data.iter() {
                            if let Some(lli_flags) = data.lli {
                                if lli_flags == lli { 
                                    inner.insert(code.clone(), data.clone());
                                }
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
            _ => {},
        }
    }
    //[4*] SSI presence filter
    if has_ssi {
        match &rinex.header.rinex_type {
            Type::ObservationData => {
                let mut rework = observation::Record::new();
                for (epoch, (ck,data)) in rinex.record.as_obs().unwrap().iter() {
                    let mut map : HashMap<Sv, HashMap<String, observation::ObservationData>> = HashMap::new();
                    for (sv, data) in data.iter() {
                        let mut inner : HashMap<String, observation::ObservationData> = HashMap::new();
                        for (code, data) in data.iter() {
                            if let Some(ssi_flags) = data.ssi {
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
            _ => {},
        }
    }
    //[4*] SSI filter
    if let Some(ssi) = ssi {
        match &rinex.header.rinex_type {
            Type::ObservationData => {
                let mut rework = observation::Record::new();
                for (epoch, (ck,data)) in rinex.record.as_obs().unwrap().iter() {
                    let mut map : HashMap<Sv, HashMap<String, observation::ObservationData>> = HashMap::new();
                    for (sv, data) in data.iter() {
                        let mut inner : HashMap<String, observation::ObservationData> = HashMap::new();
                        for (code, data) in data.iter() {
                            if let Some(ssi_value) = data.ssi {
                                if ssi_value >= ssi { 
                                    inner.insert(code.clone(), data.clone());
                                }
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
            _ => {},
        }
    }
        
    if split {
        match rinex.split(split_epoch) {
            Ok(files) => {
                let mut offset = 1;
                for f in files {
                    if f.to_file(&format!("split-{}.txt", offset)).is_err() {
                        println!("Failed to write Split record to \"split.txt\"")
                    }
                    offset += 1
                }
            },
            Err(e) => println!("Split() ops failed with {:?}", e),
        }
    }

    if splice {
       println!("splice is WIP"); 
    }
    
    if !spec_ops {
        if header {
            if pretty {
                println!("{}", serde_json::to_string_pretty(&rinex.header).unwrap())
            } else {
                println!("{}", serde_json::to_string_pretty(&rinex.header).unwrap())
            }
        }
        if epoch_display {
            let e : Vec<_> = match rinex.header.rinex_type {
                Type::ObservationData => rinex.record.as_obs().unwrap().keys().collect(),
                Type::NavigationData => rinex.record.as_nav().unwrap().keys().collect(),
                Type::MeteoData => rinex.record.as_meteo().unwrap().keys().collect(),
            };
            if pretty {
                println!("{}", serde_json::to_string_pretty(&e).unwrap())
            } else {
                println!("{}", serde_json::to_string(&e).unwrap())
            }
        }
        if obscode_display {
            match rinex.header.rinex_type {
                Type::ObservationData => {
                    let codes = rinex.header.obs_codes.unwrap();
                    if pretty {
                        println!("{}", serde_json::to_string_pretty(&codes).unwrap())
                    } else {
                        println!("{}", serde_json::to_string(&codes).unwrap())
                    }
                },
                Type::MeteoData => {
                    let codes = rinex.header.met_codes.unwrap();    
                    if pretty {
                        println!("{}", serde_json::to_string_pretty(&codes).unwrap())
                    } else {
                        println!("{}", serde_json::to_string(&codes).unwrap())
                    }
                },
                Type::NavigationData => { // (NAV) special procedure
                    let r = rinex.record.as_nav().unwrap();
                    let mut map : HashMap<String, Vec<String>> = HashMap::new();
                    for (_, sv) in r.iter() {
                        let mut codes : Vec<String> = Vec::new();
                        for (sv, data) in sv {
                            let codes : Vec<String> = data
                                .keys()
                                .map(|k| k.to_string())
                                .collect();
                            map.insert(
                                sv.constellation.to_3_letter_code().to_string(), 
                                codes);
                        }
                    }
                    if pretty {
                        println!("{}", serde_json::to_string_pretty(&map).unwrap())
                    } else {
                        println!("{}", serde_json::to_string(&map).unwrap())
                    }
                },
            };
        }
        if !epoch_display && !obscode_display && !header { 
            match rinex.header.rinex_type {
                Type::ObservationData => {
                    let r = rinex.record.as_obs().unwrap();
                    if pretty {
                        println!("{}", serde_json::to_string_pretty(r).unwrap())
                    } else {
                        println!("{}", serde_json::to_string(r).unwrap())
                    }
                },
                Type::NavigationData => {
                    let r = rinex.record.as_nav().unwrap();
                    if pretty {
                        println!("{}", serde_json::to_string_pretty(r).unwrap())
                    } else {
                        println!("{}", serde_json::to_string(r).unwrap())
                    }
                },
                Type::MeteoData => {
                    let r = rinex.record.as_meteo().unwrap();
                    if pretty {
                        println!("{}", serde_json::to_string_pretty(r).unwrap())
                    } else {
                        println!("{}", serde_json::to_string(r).unwrap())
                    }
                },
            }
        }
    }
}// for all files
    
    // Merge() opt
    for i in 0..to_merge.len() {
        if merged.merge(&to_merge[i]).is_err() {
            println!("Failed to merge {} into {}", filepaths[i], filepaths[0])
        }
    }

    if merge {
        if header {
            if pretty {
                println!("{}", serde_json::to_string_pretty(&merged.header).unwrap())
            } else {
                println!("{}", serde_json::to_string(&merged.header).unwrap())
            }
        }
        if obscode_display {
            let obs = merged.header.obs_codes
                .as_ref()
                .unwrap();
            if pretty {
                println!("{}", serde_json::to_string_pretty(&obs).unwrap())
            } else {
                println!("{}", serde_json::to_string(&obs).unwrap())
            }
        }
        if epoch_display {
            let e : Vec<_> = match merged.header.rinex_type {
                Type::ObservationData => merged.record.as_obs().unwrap().keys().collect(),
                Type::NavigationData => merged.record.as_nav().unwrap().keys().collect(),
                Type::MeteoData => merged.record.as_meteo().unwrap().keys().collect(),
            };
            if pretty {
                println!("{}", serde_json::to_string_pretty(&e).unwrap())
            } else {
                println!("{}", serde_json::to_string(&e).unwrap())
            }
        }
        if merge {
            if merged.to_file("merged.txt").is_err() {
                println!("Failed to write MERGED RINEX to \"merged.txt\"")
            }
        }
    }

}// main
