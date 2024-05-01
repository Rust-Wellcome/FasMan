# FastaManipulator

This is a re-write of the current fasta manipulation scripts I've written whilst at ToL, as well as adding some functionality needed for future projects.

Currently, this program has the following arguments:

-   yaml_validator

    This validates a given yaml against the TreeVal yaml standard. This is specific to the TreeVal pipeline.
    This command will go through the yaml and validate file and directory paths as well as files are in the expected format.

    `validateyaml ${PATH TO YAML} --verbose {DEFAULT FALSE} --output ${OUTPUT LOCATION OF LOGS}`

-   map_headers

    This command generates a mapping file of a given fasta files headers to new names, this standarises headers to a small form factor with no special characters (by default this is 'FMM'). The fasta file is then copied with the new mapped headers in place. The output directory folder must already exist.

    `mapheaders --fasta-file ${PATH TO FASTA} --output-directory ${OUTPUT LOCATION} --replace-with ${STRING FOR NEW HEADER}`

-   remap_headers

    This compliments the above function by using the above generated map file to regenerate the original headers.

-   split_by_count (NOT YET WRITTEN)

    This command will generate a directory of files made up of a user given number of sequences from the input fasta. This is useful when generating geneset data for TreeVal use or sub-setting data in a non-random manner.
    The count will be the upper limit, as there will be a left over number of records.

    `splitbycount --fasta-file ${PATH TO FASTA} --output-directory ${OUTPUT LOCATION} --count {NUMBER OF FASTA RECORDS PER FILE}`

-   split_by_size (NOT YET WRITTEN)

    This command will generate a directory of files, of user given size (in MB), generated from the input fasta. This is useful for consistent sizes of files used in geneset alignments.
    The mem-size will be approximate as some records may exceed the chosen size, inversely, there will be a final file collecting small sequences which do not meet the limit.

    `splitbysize --fasta-file ${PATH TO FASTA} --output-directory ${OUTPUT LOCATION} --mem-size ${SIZE OF OUTPUT FILES IN Mega Bytes}`

-   curate

    Use a tpf and fasta file to generate a curated fasta file.

    `curate --fasta input.fasta --tpf input.tpf --output curated.fasta`

-   filterfasta

    Given a comma seperated list, create a new fasta file removing the named sequence.

    `filterfasta -f input.fasta -l "SUPER_1,SUPER_2" -o output.fasta`

-   mergehaps (NOT YET WRITTEN)

    Given two fasta files, generate 1 merged fasta file and rename the scaffs.

    `mergehaps -p primary.fasta -s secondary.fasta -n PRI/HAP -o merged.fasta`

-   profile (NOT YET WRITTEN)

    Profile a given fasta file:

    -   GC percentage per scaffold + counts
    -   GC percentage whole genome

If there are other options that would be useful to any other teams, leave a message or issue.
