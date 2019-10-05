use super::text_translator::TextTranslator;
use std::fs::File;
use std::io;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use zip::{ZipArchive, ZipWriter};

pub struct PPTXTranslator<'a> {
    path: &'a Path,
    output_path: PathBuf,
}

impl<'a> PPTXTranslator<'a> {
    pub fn new(path: &'a str, output_path: &'a str) -> Self {
        let p = Path::new(path);
        let output = PathBuf::from(output_path);
        PPTXTranslator {
            path: p,
            output_path: output,
        }
    }

    fn convert_archive<A: Read + io::Seek, B: Write + io::Seek>(
        archive: &mut ZipArchive<A>,
        output_archive: &mut ZipWriter<B>,
    ) -> io::Result<()> {
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            match file.name() {
                f if f.find("ppt/slides/slide").is_some() => {
                    let mut buff = String::new();
                    file.read_to_string(&mut buff)?;
                    let translator = TextTranslator::new();
                    let converted = translator.translate(&buff);
                    let options =
                        zip::write::FileOptions::default().compression_method(file.compression());
                    output_archive.start_file(file.name(), options)?;
                    output_archive.write_all(converted.as_bytes())?;
                }
                _ => {
                    let mut buffer = Vec::with_capacity(file.size() as usize);
                    //                    file.read_exact(&mut buffer);
                    file.read_to_end(&mut buffer)?;
                    let options =
                        zip::write::FileOptions::default().compression_method(file.compression());
                    output_archive.start_file(file.name(), options)?;
                    output_archive.write_all(&buffer)?;
                }
            }
        }
        Ok(())
    }

    pub fn translate(&self) -> io::Result<()> {
        let f = File::open(self.path)?;
        let mut archive = ZipArchive::new(f).unwrap();
        let output_file = File::create(&self.output_path)?;
        let mut output_archive = ZipWriter::new(output_file);

        Self::convert_archive(&mut archive, &mut output_archive)?;

        output_archive.finish()?;

        Ok(())
    }
    pub fn from_stream<R: io::Read+io::Seek>(reader: &mut R) -> io::Result<Vec<u8>> {
        let mut archive = ZipArchive::new(reader)?;
        let mut mem = Cursor::new(Vec::new());
        {
            let mut output_archive = ZipWriter::new(&mut mem);
            Self::convert_archive(&mut archive, &mut output_archive)?;
            output_archive.finish()?;
        }
        let mut v = Vec::new();
        mem.set_position(0);
        mem.read_to_end(&mut v)?;
        Ok(v)
    }
}

#[cfg(test)]
mod text_translation_tests {
    use super::PPTXTranslator;

    // Test with your files
    //    #[test]
    //    fn test_pptx() {
    //        let translator = PPTXTranslator::new("", "");
    //        translator.translate();
    //    }
}
