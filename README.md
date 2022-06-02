# RINEX Cli 
Command line tool to handle, manage and analyze RINEX files

[![crates.io](https://img.shields.io/crates/v/rinex-cli.svg)](https://crates.io/crates/rinex-cli)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/rinex-cli/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/rinex-cli/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/rinex-cli.svg)](https://crates.io/crates/rinex-cli)    
[![Rust](https://github.com/gwbres/rinex-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/gwbres/rinex-cli/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/rinex-cli/badge.svg)](https://docs.rs/rinex-cli/badge.svg)

This command line interface implements the latest 
[Rinex crate](https://crates.io/crates/rinex)
and allows easy RINEX files manipulation.

## Getting started

Run with `cargo`

```bash
cargo run
```

Filepath : -fp or --filepath   
Lets you select local RINEX files:

```bash
cargo run --filepath /tmp/amel010.21g
cargo run --filepath /tmp/amel010.21g,/mnt/CBW100NLD_R_20210010000_01D_MN.rnx
```

Compressed OBS are natively supported.

&#9888; for V > 2 RINEX OBS, this parser expects
single line epochs (resulting from RNX2CRX compression). 

Epoch flag : -e    
Using this flag will print all encountered epochs.

Epoch-ok: --epoch-eok
Will restrict all maniuplations to epochs that have an `EpochFlag::Ok` flag
associated to them.

## GNSS constellation filter

Constellation : -c or --constellation   
lets you filter and retain satelllite vehicules that
have are associated to these constellations.

For example:
```bash
cargo run --filepath amel010.21g -c GLO
```

will retain only Glonass vehicules

```bash
cargo run -fp amel010.21g -c GLO,E
```

will retain both Glonass and Galileo
satellite vehicules

Constellation identification supports:
* standard 3 letter RINEX identification code
* standard 1 character RINEX identification code
* full name

## Satellite vehicule filters

Sv: -v or --vehicule  
lets you select matching satellite vehicules

```bash
cargo run -fp amel010.21g -v R04
```

Only R04 to be retained

```bash
cargo run -fp amel010.21g --vehicule R04,E10
```

will study R04 and E10

## Navigation data

Nav : -n or --navigation   
Lets you select NAV file data fields

```bash
cargo run -fp amel010.21g --nav iode,health
```

Refer to the known
[NAV fields database](https://github.com/gwbres/rinex/blob/main/navigation.json)

## Observation data
Obs : -o or --observation   
Lets you select OBS code of interests.   
These work for both METEO and OBS data

```bash
cargo run -fp CBW100NLD_R_20210010000_01D_MN.rnx -o L1C,S1P 
```

Codes must be valid and encountered OBS codes.

## Output format

Default format is purely stdout.   

--csv lets you output all the data into csv format   
Will create 1 csv file per RINEX file, same name, created locally.   
--prefix moves the output file location   
--plot will not print but plot all data using this lib ...   
Will create 1 plot per file

## TODO

[ ] Pretty print?
[ ] graphix pleaz
