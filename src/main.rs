use glob::glob;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    table_header()?;
    table_delim()?;
    make_count_table()?;

    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "grep -v '\\*' merged_idx_table.txt> idx_count_table.txt"
        ))
        .output()?;

    std::fs::remove_file("header")?;
    std::fs::remove_file("delim")?;
    std::fs::remove_file("output")?;
    std::fs::remove_file("output2")?;
    std::fs::remove_file("merged_idx_table.txt")?;

    Ok(())
}

fn table_header() -> Result<(), std::io::Error> {
    let mut filenames: Vec<String> = Vec::new();
    // Create the header from filenames
    for entry in glob("*.idx.txt").map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
        let path = entry
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .display()
            .to_string();
        let file_stem = path.split('.').next().unwrap().to_string();
        filenames.push(file_stem);
    }

    let mut header_file = BufWriter::new(File::create("header")?);
    writeln!(header_file, "{}", filenames.join("\t"))?;

    Ok(())
}

fn table_delim() -> Result<(), std::io::Error> {
    // Create rownames for merged count table
    let mut delim_file = BufWriter::new(File::create("delim")?);
    let first_file = glob("./*.idx.txt")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .next()
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "No files found"))?;
    let file = File::open(first_file.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let rowname = line.split('\t').next().unwrap();
        writeln!(delim_file, "{}", rowname)?;
    }

    Ok(())
}

fn make_count_table() -> io::Result<()> {
    let mut temp_files = Vec::new();
    let mut index = 0;
    for entry in glob("./*.idx.txt").map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
        let path = entry
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .display()
            .to_string();
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        index += 1;
        let temp_file_name = format!("{}__{}.temp", &path, index);
        let mut temp_file = BufWriter::new(File::create(&temp_file_name)?);
        temp_files.push(temp_file_name.clone());

        for line in reader.lines() {
            let line = line?;
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() > 2 {
                writeln!(temp_file, "{}", cols[2])?;
            }
        }
    }

    let temp_files_str = temp_files.join(" ");
    Command::new("sh")
        .arg("-c")
        .arg(format!("paste -d'\\t' {} > output", temp_files_str))
        .output()?;

    // Concatenate delim, output into the final table
    Command::new("sh")
        .arg("-c")
        .arg(format!("paste delim output > output2"))
        .output()?;

    Command::new("sh")
        .arg("-c")
        .arg(format!("cat header output2  > merged_idx_table.txt"))
        .output()?;

    // Cleanup temp files
    for file in temp_files {
        std::fs::remove_file(file)?;
    }

    Ok(())
}
