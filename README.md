# RINEX Cli 
Command line tool to handle, manage and analyze RINEX files

[![crates.io](https://img.shields.io/crates/v/rinex-cli.svg)](https://crates.io/crates/rinex-cli)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/rinex-cli/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/rinex-cli/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/rinex-cli.svg)](https://crates.io/crates/rinex-cli)    
[![Rust](https://github.com/gwbres/rinex-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/gwbres/rinex-cli/actions/workflows/rust.yml)

## RINEX

Why this tool ?

RINEX files are very common, they are used in 
* GNSS applications :artificial_satellite:
* Timing applications :clock1:
* Astronomy :satellite:
* Space applications :rocket: 
* Navigation applications :earth_americas:

RINEX files are complex, several kinds exist and they differ a lot from one another.  
This tool is powerful enough to manage almost all revisions and most common RINEX files,
without compromising ease of use.

## Notes on RINEX data

### Supported RINEX

* Observation Data (OBS)
* Compressed Observation Data (CRINEX)
* Navigation Data (NAV)
* Meteo Data (MET)

### File naming conventions

File names are disregarded by these tools, you can analyze
& parse files that do not follow naming conventions.

### Compressed files

RINEX files are most of the time compressed.

This tool supports CRINEX (compressed RINEX) natively. You can
pass a CRINEX and parse it directly.

This tool does not support extra compression (like .gz for instance).
It is up to the user to decompress these files prior analysis.

This tool is focused on data extraction & analysis, if you want to perform
file operations like `RNX2CRX` (compression) and `CRX2RNX` (decompression),
prefer [this tool](https://github.com/gwbres/hatanaka) instead.

### RINEX Revisions

Many RINEX revisions exist

* Supported RINEX are tested for 1.00 ⩽ v < 4.0    
* v = 4.0 and newer should work but is either
not fully tested, not garanteed or restrictions may apply.

Refer to the [RINEX framework documentation](https://crates.io/crates/rinex),
for detailed information.

## Getting started

This application is managed by `cargo`, therefore `cargo run` is one way to run it

```bash
cargo run -- --help
```

Command line arguments order does not matter.  
(Input) `filepath` is the only mandatory argument, other flags are optionnal.
`Help` menu tells you which argument has a shortenned version,
here is an example on how to use a shortenned argument:

```bash
cargo run --filepath /tmp/amel010.21g
cargo run -f /tmp/amel010.21g
```

Using wrong arguments like a `GNSS` or `Sv` filter on Meteo Data for instance,
will not cause a panic.

Arguments that support an array of value are described using comma separation :

```bash
cargo run -f /tmp/amel010.21g # analyze only 1 file
cargo run -f \ # analyze both files
    /tmp/amel010.21g,/mnt/CBW100NLD_R_20210010000_01D_MN.rnx
```

## Output format

This tool display everything in the terminal (`stdout`).
One should pipe the output data to a file in order to store it.

This tool uses JSON format to expose data, which makes it easy
to import into external tool for further calculations and processing,
like python scripts for instance.

At any moment, add the `--pretty` option to make this more readable if desired.
Output is still valid JSON.

## Epoch identification 

`--epoch` or `-e`
is used to export the identified epochs (sampling timestamps)
in the given RINEX records.
When we extract data, we always associate it to a sampling timestamp.
Therefore, this flag should only be used if the user studies
epoch events or sampling events specifically.

Example :

```bash
cargo run -- -f /tmp/data.obs --epoch --pretty
cargo run -- -f /tmp/data.obs -e > /tmp/epochs.json
```

## OBS / DATA code identification

`--obscodes` or `-o`
is used to identify which data codes (not necesarily OBS..) 
are present in the given records.
This macro is very useful because it lets the user
understand which data (physics) is present and we can build
efficient data filter from that information

```bash
cargo run -- -f /tmp/data.obs --obscodes --pretty
cargo run -- -f /tmp/data.obs -o > /tmp/data-codes.json
```

This flag name can be misleading as it is possible to use this
flag to identify NAV data field also !

## Record resampling

Record resampling is work in progress !

## Epoch filter

Some RINEX files like Observation Data 
associate an epoch flag to each epoch.  
A non `Ok` epoch flag describes a special event
or external pertubations that happened at that sampling
date. We provide the following arguments to
easily discard unusual events or focus on them to
figure things out:

* --epoch-ok : will retain only valid epochs (EpochFlag::Ok).
This can be a quick way to reduce the amount of data

* --epoch-nok : will retain only non valid epoch (!EpochFlag::Ok). 

Example :

```bash
cargo run -- -f /tmp/data.obs -c C1C,C2C # huge set
cargo run -- -f /tmp/data.obs --epoch-ok -c C1C,C2C # reduce set 
cargo run -- -f /tmp/data.obs --epoch-nok -c C1C,C2C # focus on weird events 
```

## Constellation filter

User can filter some GNNS constellations out, with `--constellation`, 
only the described systems will be kept:

```shell
cargo run -- -f /tmp/data.obs --epoch-ok \
    --constellation GPS
cargo run -- -f /tmp/data.obs --epoch-ok \
    --constellation GPS,GAL
```

## Satellite vehicule filter

User can focus on specific satellite vehicules, only
specific vehicules will be kept:

Example:

```shell
cargo run -- -f /tmp/data.obs --epoch-ok \
    --sv R01,R2,E06,E24,R24
cargo run -- -f /tmp/data.obs --constellation GLO,GAL --epoch-ok \
    --sv R01,R2,E06,E24,R24
```

## Data filter

We use the -c or --code argument to filter data out
and only retain data of interest.

* Observation / meteo data: we use the OBS code directly
* Navigation data: we use the official identification keyword,
refer to the known
[NAV database](https://github.com/gwbres/rinex/blob/main/navigation.json)


Example:

```bash
cargo run -f CBW100NLD_R_20210010000_01D_MN.rnx -c L1C,S1P 
```

## Cummulated filters

Because all arguments can be cummulated, one can 
create efficient data filter and focus on data of interest: 

```bash
cargo run -f CBW100NLD_R_20210010000_01D_MN.rnx \
    --lli 0 # "OK" \
        --ssi 5 # not bad \
            -c C1C,C2C,C1X # PR measurements only :) \
                --sv G01,G2,G24,G25 # GPS focus !
```

## Observation Data specific filters

Observation Data may comprise an LLI flag that describes
RX conditions, and an SSI flag that depicts the RX SNR.

Some flags are available to interact and filter using these informations:

* `--has-lli` : will only retain OBS that have an LLI flag attached to them
* `--lli %d` : will only retain OBS a matching LLI flag 
* `--has-ssii` : will only retain OBS that have an SSI flag attached to them
* `--ssii %d` : will only retain OBS have an SSI flag >= %d 

for instance 0x01 means Loss of Lock at given epoch:
Exemples :

```shell
# Retain Obs + LLI
cargo run -- -f /tmp/data.obs --constellation GPS \
    --has-lli > output.txt
# Retain `Loss of Lock` events 
cargo run -- -f /tmp/data.obs --lli 1 > output.txt
# Retain >= 5 <=> at least 30 dB SNR
cargo run -- -f /tmp/data.obs --ssi 5 > output.txt
```

## `teqc` operations

This tool supports special operations that only
`teqc` supports at the moment. Therefore
it can be an efficient alternative to this program.

All of the special operations actually create an output file.

## `Merge` special operation

It is possible to perform merging operations with `-m` or `--merge`, in `teqc` similar fashion.

When merging, if analysis are to be performed, they will be performed on the resulting record.

For example:

```bash
cargo run -f file1.rnx,/tmp/file2.rnx
```

## `Split` special operation

It is possible to split given RINEX files into two.

`Split` has two behaviors

* (1) if given RINEX is a `merged` RINEX (either by `teqc` or this tool),
we split the record at the epoch (timestamps) where records were previously merged

* (2) otherwise, the user is expected to describe a timestamp (`epoch`) 
at which we will split the record.

### Splitting a previously merged record

```bash
# Merge two RINEX toghether
cargo run -f /tmp/file1.rnx,/tmp/file2.rnx -m --output /tmp/merged.rnx
# Split resulting RINEX
cargon run -f /tmp/merged.rnx --split --output /tmp/split1.rnx,/tmp/split2.rnx 
```

When splitting a merged RINEX, the header section is simply
copied into both results.

### Splitting record

If User provides an `epoch`, the tool will try to locate the
given timestamp and perform `split()` at this date & time.

Two description format are supported, for the user to describe a sampling
timestamp:

* "YYYY-MM-DD HH:MM:SS" : Datetime description and EpochFlag::Ok is assumed
* "YYYY-MM-DD HH:MM:SS X" : Datetime description where X describes the EpochFlag integer value.
Refer to RINEX standards for supported Epoch flag values 

The tool identifies matching timestamp by comparing the datetime field AND
the flag field. They both must match.

Example :

```bash
# Split a previously merged record
cargo run -f /tmp/merged.rnx --split \
    --output /tmp/file1.rnx,/tmp/file2.rnx

# Split a record at specified timestamp,
# don't forget the \" encapsulation \" ;)
cargo run -f /tmp/data.rnx --split "2022-06-03 16:00:00" \
    --output /tmp/file1.rnx,/tmp/file2.rnx

# Split a record at specified timestamp with precise Power Failure event
# don't forget the \" encapsulation \" ;)
cargo run -f /tmp/data.rnx --split "2022-06-03 16:00:00 1" \
    --output /tmp/file1.rnx,/tmp/file2.rnx
```
