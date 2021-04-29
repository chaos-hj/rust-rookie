use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

type MyResult = Result<(), std::io::Error>;

fn main() -> MyResult {
    unpack()?;
    compress()?;
    compress1()?;
    Ok(())
}

fn unpack() -> MyResult {
    let path = "unpack.tar.gz";
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack("./unpack")?;
    Ok(())
}

fn compress() -> MyResult {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    let tar_gz = File::create("compress.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("log-back", "logs")?;
    Ok(())
}


fn compress1() -> MyResult {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    let tar_gz = File::create("compress1.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir("log-back", "logs")?;
    Ok(())
}
