# FastaManipulator

This is a re-write of the current fasta manipulation scripts i've written whilst at ToL. This script will allow for:
- Splitting of fasta into user given header-sequence pairs per file, e.g., `--count 1000`
- Splitting of fasta into user given memory sizes per file, e.g., `--file-size 50MB`
- Reformatting of the header string, for now this will be into a simple code we use in ToL. More complex regex flag could come at later date, e.g, `--header-regex {SOME REG}`

TODO:
- Change the arg structure into:
    - `split-count -f test.fasta -c 1000`
    - `split-mem -f test.fasta -s 50MB`
    - `reformat-head -f test.fasta --header-regex`