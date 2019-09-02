use super::text_translator::TextTranslator;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::{ZipArchive, ZipWriter};

pub struct DocXTranslator<'a> {
    path: &'a Path,
    output_path: PathBuf,
}

impl<'a> DocXTranslator<'a> {
    pub fn new(path: &'a str) -> Self {
        let p = Path::new(path);
        DocXTranslator {
            path: p,
            output_path: p.with_file_name(std::ffi::OsString::from(&String::from(
                p.file_stem().unwrap().to_string_lossy() + " converted.docx",
            ))),
        }
    }
    pub fn translate(&self) -> io::Result<()> {
        let f = File::open(self.path)?;
        let mut archive = ZipArchive::new(f).unwrap();
        println!("{:?}", self.output_path.as_os_str());
        let mut output_file = File::create(&self.output_path)?;
        let mut output_archive = ZipWriter::new(output_file);

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            match file.name() {
                "word/document.xml" => {
                    let mut buff = String::new();
                    file.read_to_string(&mut buff);
                    let translator = TextTranslator::new();
                    let converted = translator.translate(&buff);
                    let options =
                        zip::write::FileOptions::default().compression_method(file.compression());
                    output_archive.start_file(file.name(), options);
                    output_archive.write(converted.as_bytes());
                }
                _ => {
                    let mut buffer = Vec::with_capacity(file.size() as usize);
                    //                    file.read_exact(&mut buffer);
                    file.read_to_end(&mut buffer);
                    let options =
                        zip::write::FileOptions::default().compression_method(file.compression());
                    output_archive.start_file(file.name(), options);
                    output_archive.write(&buffer);
                }
            }
        }
        output_archive.finish();

        Ok(())
    }
}

#[cfg(test)]
mod text_translation_tests {
    use super::DocXTranslator;

    // Test with your files
    //    #[test]
    //    fn test_docx() {
    //        let t = DocXTranslator::new("");
    //        assert!(t.translate().is_ok());
    //    }
}