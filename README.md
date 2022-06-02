# RINEX Cli 
Command line tool to handle, manage and analyze RINEX files

[![crates.io](https://img.shields.io/crates/v/rinex-cli.svg)](https://crates.io/crates/rinex-cli)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/rinex-cli/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/rinex-cli/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/rinex-cli.svg)](https://crates.io/crates/rinex-cli)    
[![Rust](https://github.com/gwbres/rinex-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/gwbres/rinex-cli/actions/workflows/rust.yml)

This command line interface implements the latest 
[Rinex crate](https://crates.io/crates/rinex)
and allows easy RINEX files manipulation.

## RINEX

Why this tool ?

RINEX are very common worldwide, they are used in GNSS, timing and navigation related applications
:rocket: :satellite: :earth_americas:.

RINEX are complex files, several kinds exist, they differ a lot from one another.  
This tool is powerful enough to manage almost all revisions and most common RINEX files,
without compromising ease of use.

## Compressed Data

RINEX files are huge and so, are most of the time compressed.

It is possible for this tool to analyze a Hatanaka compressed RINEX (also called CRINEX) file directly.
That means, you don't have to run extra tools like `CRX2RNX` before running an analysis.

It is not possible to analyze a .gz or other compression method stacked on top of a CRINEX
at this moment. Therefore the user should uncompress the file, for instance with "gunzip -d" prior
running an anlysis.

## Getting started

Run with `cargo`

```bash
cargo run -- --help
```

Filepath : -fp or --filepath, to select local RINEX files 

```bash
cargo run --filepath /tmp/amel010.21g
cargo run --filepath /tmp/amel010.21g,/mnt/CBW100NLD_R_20210010000_01D_MN.rnx
```

## General info
This tool currently has 3 opmodes:

* -e will print encountered epochs (sampling timestamps)
* -o can be combined to -e, will print encountered OBS codes, useful to
determine which data we can filter out

Otherwise (nominal op), -e and/or -o are not requested.

Example :

```shell
# determine available data
cargo run -- --fp /tmp/data.obs -e > info.txt
cargo run -- --fp /tmp/data.obs -o -e > info.txt
# nominal use
cargo run -- --fp /tmp/data.obs > data.txt
```

### Epoch filter

* --epoch-ok : will retain only valid epoch, ie.,
epochs that have an Epoch::Flag::Ok attached to them

* --epoch-nok : will retain only non valid epoch, ie.,
epochs that have an !Epoch::Flag::Ok attached to them

Epoch filter only makes sense on OBS data (meteo or obs).

### Satellite vehicule filter

* --sv with a comma separated list of satellite vehicule to retain

Example:

```shell
cargo run -- --filepath /tmp/data.obs --epoch-ok --sv G01,G2,E06,E24,R24 > data.txt
```

Will only retain data from GPS 1+2, GAL 6+24 and GLO 24 vehicules.


### Constellation filter

Constellation filter is not feasible at the moment

### Data filter

We use the -c or --code argument to filter data out
and only retain data of interest.

* Observation / meteo data: we use the OBS code directly
* Navigation data: we use the official identification keyword,
refero to the known
Refer to the known
[NAV fields database](https://github.com/gwbres/rinex/blob/main/navigation.json)


Example:

```bash
# Retain Carrier phase + Carrier power 
cargo run -fp CBW100NLD_R_20210010000_01D_MN.rnx -c L1C,S1P 
```

### Cumulated filter

All arguments can be cummulated,
for example:

```bash
# Retain Carrier phase + Carrier power 
cargo run -fp CBW100NLD_R_20210010000_01D_MN.rnx \
    -c C1C,C2C,C1X --sv G01,E06,G24,E24
```
will only retain pseudo range measurements for given satellite of interest.
