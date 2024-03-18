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

```
$ cargo run -- 5 ./SBOMs/ ./data-set/
.../...  
Replication multiplier 5
Source directory ./SBOMs/
Destination directory ./data-set/
Amending version:     "version": 1,
successfully wrote to ./data-set/batch1/A7ED160707AB4BC.replicate1.cdx.json
Amending version:     "version": 1,
successfully wrote to ./data-set/batch2/A7ED160707AB4BC.replicate2.cdx.json
Amending version:     "version": 1,
successfully wrote to ./data-set/batch3/A7ED160707AB4BC.replicate3.cdx.json
Amending version:     "version": 1,
successfully wrote to ./data-set/batch4/A7ED160707AB4BC.replicate4.cdx.json
Amending version:     "version": 1,
successfully wrote to ./data-set/batch5/A7ED160707AB4BC.replicate5.cdx.json
Amending name:   "name": "quarkus-2.13",
Amending documentNameSpacekey:   "documentNamespace": "https://example.com/security/data/sbom/beta/spdx/quarkus-2.13-1a6ac4c55918a44fb3bada1b7e7d12f887d67be4",
successfully wrote to ./data-set/batch1/quarkus-2.replicate1.13.json
Amending name:   "name": "quarkus-2.13",
Amending documentNameSpacekey:   "documentNamespace": "https://example.com/security/data/sbom/beta/spdx/quarkus-2.13-1a6ac4c55918a44fb3bada1b7e7d12f887d67be4",
successfully wrote to ./data-set/batch2/quarkus-2.replicate2.13.json
Amending name:   "name": "quarkus-2.13",
Amending documentNameSpacekey:   "documentNamespace": "https://example.com/security/data/sbom/beta/spdx/quarkus-2.13-1a6ac4c55918a44fb3bada1b7e7d12f887d67be4",
successfully wrote to ./data-set/batch3/quarkus-2.replicate3.13.json
Amending name:   "name": "quarkus-2.13",
Amending documentNameSpacekey:   "documentNamespace": "https://example.com/security/data/sbom/beta/spdx/quarkus-2.13-1a6ac4c55918a44fb3bada1b7e7d12f887d67be4",
successfully wrote to ./data-set/batch4/quarkus-2.replicate4.13.json
Amending name:   "name": "quarkus-2.13",
Amending documentNameSpacekey:   "documentNamespace": "https://example.com/security/data/sbom/beta/spdx/quarkus-2.13-1a6ac4c55918a44fb3bada1b7e7d12f887d67be4",
successfully wrote to ./data-set/batch5/quarkus-2.replicate5.13.json
```

```    
$ tree data-set/
data-set/
├── batch1
│   ├── A7ED160707AB4BC.replicate1.cdx.json
│   └── quarkus-2.replicate1.13.json
├── batch2
│   ├── A7ED160707AB4BC.replicate2.cdx.json
│   └── quarkus-2.replicate2.13.json
├── batch3
│   ├── A7ED160707AB4BC.replicate3.cdx.json
│   └── quarkus-2.replicate3.13.json
├── batch4
│   ├── A7ED160707AB4BC.replicate4.cdx.json
│   └── quarkus-2.replicate4.13.json
└── batch5
    ├── A7ED160707AB4BC.replicate5.cdx.json
    └── quarkus-2.replicate5.13.json
```


## scale test example ##

### Using bombastic_walker ###

#### Prepare initial SBOMs data set ####

- Copy the SBOMs files which are going to be used for replication, i.e `/SBOMs/`.
  Use only SPDX files, for now, because the CycloneDX files tested were rejected with following error: `JSON: Unsupported CycloneDX version: 1.4`
- Create a replication directory: '`/data-set/`.
- Run the replication tool : `cargo run -- 10 /SBOMs/ /data-set/`
- In each batch directory, add a metadata subdirectory containting https://github.com/trustification/trustification/tree/main/data/ds1/sbom/metadata/metadata.json file.
  This is necessary for the bombastic-walker to parse the files.
- Run the bombastice_walker on each batch directory :
  `# For example in devmode:`  
  `RUST_LOG=info cargo run -p trust bombastic walker --sink http://localhost:8082 --source /data-set/batch1/ --devmode -3`
