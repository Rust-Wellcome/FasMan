# FasMan

## A FastaManipulator script that is slowly doing more...

Originally written by @DLBPointon
Now a collaborative programming project for the Rust@Wellcome group (Sanger).

Collaborators and contributors:

-   @figueroakl - Genome Profiling
-   @stevieing - Adding tests, optimisations & CI/CD
-   @dasunpubudumal- Adding tests, optimisations & CI/CD

---

This is a re-write of the current fasta manipulation scripts I've written whilst at ToL, as well as adding some functionality needed for future projects.

Currently, this program has the following arguments.

## yaml_validator (v2)

THIS FUNCTION IS SPECIFIC TO THE TREEVAL.yaml FILE

Updated for new yaml style and now uses struct methods.

This validates a given yaml against the TreeVal yaml standard. This is specific to the TreeVal pipeline.
This command will go through the yaml and validate file and directory paths as well as files are in the expected format.

This has been tested by downloading the TreeValTinyTest data set:

```bash
curl https://tolit.cog.sanger.ac.uk/test-data/resources/treeval/TreeValTinyData.tar.gz | tar xzf -
```

`validateyaml ${PATH TO YAML}`

### TODO:

-   Add CRAM validator to the module
    -   Find equiv to `samtools quickcheck -vvv` for a report on completeness of cram.
        -   if not then it will be a secondary process (external to FasMan)
-   Better report
    -   Report should complete and if there are fails then panic! or std::process::exit("FAILED DUE TO: ...") this is so that it can be added to the Nextflow pipelines and cause them to error out at the right place, e.g, not rely on scanning the report.log throught functions in NF.

##  map_headers

This command generates a mapping file of a given fasta files headers to new names, this standarises headers to a small form factor with no special characters (by default this is 'FMM'). The fasta file is then copied with the new mapped headers in place. The output directory folder must already exist.

`mapheaders --fasta-file ${PATH TO FASTA} --output-directory ${OUTPUT LOCATION} --replace-with ${STRING FOR NEW HEADER}`

## remap_headers

This compliments the above function by using the above generated map file to regenerate the original headers.

## split_by_count

This command will generate a directory of files made up of a user given number of sequences from the input fasta. This is useful when generating geneset data for TreeVal use or sub-setting data in a non-random manner.
The count will be the upper limit, as there will be a left over number of records.

This will generate files in `{outdir}/{fasta-file.prefix}/{data_type}/{input_file_prefix}_f{file_count}_c{requested_chunk_count}-a{actual_chunk_count}.fa`

`splitbycount --fasta-file ${PATH TO FASTA} --output-directory ${OUTPUT LOCATION} --count {NUMBER OF FASTA RECORDS PER FILE} --data_type ['pep','cdna', 'cds', 'rna', 'other']`

## split_by_size

This command will generate a directory of files, of user given size (in MB), generated from the input fasta. This is useful for consistent sizes of files used in geneset alignments.
The mem-size will be approximate as some records may exceed the chosen size, inversely, there will be a final file collecting small sequences which do not meet the limit.

`splitbysize --fasta-file ${PATH TO FASTA} --output-directory ${OUTPUT LOCATION} --mem-size ${SIZE OF OUTPUT FILES IN Mega Bytes}`

## generate_csv
  THIS IS SPECIFIC TO TREEVAL AND THE STUCTURE OF THE GENESET DATA IN USE FOR IT

This function generates CSV files summarising the contents of a directory structure like shown below and saves this in csv_data dir:

```
geneset_data_dir
    |
    insect
        |
        csv_data
        |   |
        |   ApisMellifera.AMel1-data.csv
        |
        ApisMellifera
            |
            ApisMellifera.AMel1
                |
                {pep, cdna, cds, rna}
                    |
                    split.fasta files
```

## curate

Use a tpf and fasta file to generate a curated fasta file.

`curate --fasta input.fasta --tpf input.tpf --output curated.fasta`

## filterfasta

Given a comma seperated list, create a new fasta file removing the named sequence.

`filterfasta -f input.fasta { -l "SUPER_1,SUPER_2" | -c names.lst } -o output.fasta`

## mergehaps (NOT YET WRITTEN)

Given two fasta files, generate 1 merged fasta file and rename the scaffs.

`mergehaps -p primary.fasta -s secondary.fasta -n PRI/HAP -o merged.fasta`

## profile (IN PROGRESS)

Profile a given fasta file:

-   GC percentage per scaffold + counts
-   GC percentage whole genome
-   N50 and N90
-   L50
-   GAP count and length (summary with average length)

`profile -f input.fasta -o outdir`

# Notes
If there are other options that would be useful to any other teams, leave a message or issue.
