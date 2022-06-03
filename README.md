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

RINEX are very common worldwide, they are used in 
GNSS :artificial_satellite:, 
timing :clock1: :satellite: 
and navigation :rocket: :earth_americas: 
applications.

RINEX are complex files, several kinds exist, they differ a lot from one another.  
This tool is powerful enough to manage almost all revisions and most common RINEX files,
without compromising ease of use.

## Notes on data & RINEX

Read this section before getting started

## Compressed Data

RINEX files are huge and so, are most of the time compressed.

It is possible for this tool to analyze a Hatanaka compressed RINEX (also called CRINEX) file directly.
That means, you don't have to run extra tools like `CRX2RNX` before running an analysis.

It is not possible to analyze a .gz or other compression method stacked on top of a CRINEX
at this moment. Therefore the user should uncompress the file, for instance with "gunzip -d" prior
running an anlysis.

## Supported RINEX

* Observation Data (OBS)
* Compressed Observation Data (CRINEX)
* Navigation Data (NAV)
* Meteo Data (MET)

## Supported Revisions

* 1.00 ⩽ v < 4.0    Tested 
*             v = 4.0    refer to file type specific pages

## Getting started

You can run the application with `cargo` for instance

```bash
cargo run -- --help
```

Command line arguments order does not matter.  
(Input) `filepath` is the only mandatory argument, all other are optionnal.
`Help` menu tells you which argument has a shortenned version,
here is an example on how to use a shortenned argument:

```bash
cargo run --filepath /tmp/amel010.21g
cargo run -fp /tmp/amel010.21g
```

Some arguments, like `filepath` or `obscodes` can take an array of values.
In this case, we use comma separated enumeration like this:

```bash
cargo run -fp /tmp/amel010.21g,/mnt/CBW100NLD_R_20210010000_01D_MN.rnx
```

This tool currently has 4 modes of operation:

* (1) -e will print encountered epochs (sampling timestamps) to stdout.
The user can pipe stdout to a file to save this data.

* (2) -o will print encountered OBS codes to stdout. 
The user can pipe stdout to a file to save this data.
If we're dealing with NAV file(s), we actually print the data identification codes

* (1+2) -e and -o can be combined and used at the same time

* (3) when -e and -o are not passed, this tool will 
print the extracted / filtered data to stdout once again.
To learn how to filter data efficiently, refer to the following examples 

* (4) when -m or --merge is passed, a `merging` operation is to be performed.
It is possible to perform -e (1), -o (2) (1+2) or or any (3) analysis on the resulting merged RINEX file.
When merging, we aim at creating a new valid RINEX file. 
In this special case, an output file is created. 

Example :

```shell
# Print encountered timestamps 
cargo run -- --fp /tmp/data.obs -e
# Print encountered timestamps + OBS codes 
cargo run -- --fp /tmp/data.obs -o -e
# Dump into a file 
cargo run -- --fp /tmp/data.obs -o > infos.txt

# nominal use: record is printed (without filtering)
cargo run -- --fp /tmp/data.obs > data.txt

# nominal use: record is printed (with some filters)
cargo run -- --fp /tmp/data.obs \
    -sv G01,E6,R24,R3 -c C1C,C1X > data.txt

# special opmode
cargo run -- --fp /tmp/data1.obs,/tmp/data2.obs 
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

### LLI : RX condition filter

Observation data might have LLI flags attached to them.  
It is possible to filter data that have a matching LLI flag,
for instance 0x01 means Loss of Lock at given epoch:

```shell
cargo run -- -fp /tmp/data.obs --lli 1 --sv R01 > output.txt
```

### SSI: signal "quality" filter

Observation data might have an SSI indication attached to them.
It is possible to filter data according to this value and
retain only data with a certain "quality" attached to them.

For example, with this value, we only retain data with SSI >= 5
that means at least 30 dB SNR 

```shell
cargo run -- -fp /tmp/data.obs --ssi 1 --sv R01 > output.txt
```

### Data filter

We use the -c or --code argument to filter data out
and only retain data of interest.

* Observation / meteo data: we use the OBS code directly
* Navigation data: we use the official identification keyword,
refer to the known
[NAV database](https://github.com/gwbres/rinex/blob/main/navigation.json)


Example:

```bash
# Retain Carrier phase + Carrier power 
cargo run -fp CBW100NLD_R_20210010000_01D_MN.rnx -c L1C,S1P 
```

### Cummulated filter

All arguments can be cummulated,
for example:

```bash
cargo run -fp CBW100NLD_R_20210010000_01D_MN.rnx \
    --lli 0 # "OK" \
        --ssi 5 # not bad \
            -c C1C,C2C,C1X # PR measurements \
                --sv G01,E06,G24,E24 # focus
```

## `Merge` special operation

It is possible to perform merging operations with `-m` or `--merge`, in `teqc` similar fashion.

When merging, if analysis are to be performed, they will be performed on the resulting record.

For example:

```shell
# (1)
cargo run -fp /tmp/file1.obs,/tmp/file2.obs -o -e > infos.txt
# is identical to (2)
cargo run -fp /tmp/file1.obs,/tmp/file2.obs -m -o -e > infos.txt
```

In (1) we perform -o and -e obscodes and timestamps extraction.   
In (2) similar operation is performed but after merging both files.
