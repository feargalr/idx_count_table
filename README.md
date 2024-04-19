# IDX count table
Creates a count table from multiple samtools [idx stats](https://www.htslib.org/doc/samtools-idxstats.html) outputs.
idx stats retrieves and prints stats from a BAM file. This will build a count table from the third column, # mapped read segments. 
This is primarily for a case where the reference aligned to is multiple different sequences and you want to know the total count for each. 

## Usage
To download and use a pre-built version

'''bash
git clone https://github.com/feargalr/idx_count_table.git

## Execute in a directory with samtools idxstats outcome
pathtorepo/bin/idx_count_table
'''
