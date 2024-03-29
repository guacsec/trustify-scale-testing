# scale-testing
Utility for testing trustification at scale.

This tool is to help replicating existing SBOMs (SPDX or CycloneDX) file in order to augment an existing data set by multipling the number SBOMs files.

For instance let's say we have a total of 1000 SBMS (500 SPDX and 500) and we'd like to obtain a total of 10K SBOMs files for our scale test, so we can run the tool using a replication size of 10. 

The tool replicates existing SBOMs, by copying each file content and change its file name and its key records.


## Usage ##
After installing trustification/scale-testing repo,

We can run the tool, by providing the size of the replication, the source directory and the destination directory :

`cargo run -- 10 /SBOMs/ /data-set/`

The latter will replicate 10 times each SBOM file available in /SBOMs/.

Each replicated SBOM file will be created under its corresponding batch directory under `/data-set`.


## Example ##

```sh
$ cargo run -- 2 ./SBOMs ./data-set
   Compiling scale-testing v0.1.0 (/home/gildub/github.com/gildub/scale-testing)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/scale-testing 2 ./SBOMs ./data-set`
Replication multiplier 2
Source directory ./SBOMs
Destination directory ./data-set
successfully wrote to metadata file
successfully wrote to metadata file
Amending version:     "version": 1,
successfully wrote to ./data-set/batch1/A7ED160707AB4BC.replicate1.cdx.json
Amending version:     "version": 1,
successfully wrote to ./data-set/batch2/A7ED160707AB4BC.replicate2.cdx.json
Amending name:   "name": "quarkus-2.13",
Amending documentNameSpacekey:   "documentNamespace": "https://access.redhat.com/security/data/sbom/beta/spdx/quarkus-2.13-1a6ac4c55918a44fb3bada1b7e7d12f887d67be4",
successfully wrote to ./data-set/batch1/quarkus-2.replicate1.13.json
Amending name:   "name": "quarkus-2.13",
Amending documentNameSpacekey:   "documentNamespace": "https://access.redhat.com/security/data/sbom/beta/spdx/quarkus-2.13-1a6ac4c55918a44fb3bada1b7e7d12f887d67be4",
successfully wrote to ./data-set/batch2/quarkus-2.replicate2.13.json
```

```sh
$ tree data-set/
data-set/
├── batch1
│  ├── metadata
│  │  └── metadata.json
│  ├── A7ED160707AB4BC.replicate1.cdx.json
│  └── quarkus-2.replicate1.13.json
└── batch2
    ├── metadata
    │  └── metadata.json
    ├── A7ED160707AB4BC.replicate2.cdx.json
    └── quarkus-2.replicate2.13.json

5 directories, 6 files
```

## scale test example

### Using bombastic_walker

#### Prepare initial SBOMs data set

Provide initial set of SBOMs files to be replicated, i.e `/SBOMs/`.
Use only SPDX files, for now, because the CycloneDX files tested were rejected with following error: `JSON: Unsupported CycloneDX version: 1.4`

#### Replicate SBOMs

Run the replication tool to multiply your existing SBOM files set

`cargo run -- 10 /SBOMs /data-set`

#### Use the replicated SBOMs

The bombastic_walker could exploit each replicated SBOMs batch, for example in devmode :  

`RUST_LOG=info cargo run -p trust bombastic walker --sink http://localhost:8082 --source /data-set/batch1/ --devmode -3`
