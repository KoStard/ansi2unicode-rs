use super::text_translator::TextTranslator;
use ropey::Rope;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

pub struct TXTTranslator<'a> {
    path: &'a Path,
}

impl<'a> TXTTranslator<'a> {
    pub fn new(path: &'a str) -> Self {
        TXTTranslator {
            path: Path::new(path),
        }
    }
    pub fn translate_with_ropey(&self) -> io::Result<()> {
        let mut f = Rope::from_reader(File::open(&self.path)?)?;
        let translator = TextTranslator::new();
        let mut pos = 0;
        for i in 0..f.len_lines() {
            let line = f.line(i);
            let converted = translator.translate_from_ropey_chars(line.chars());
            let l = line.len_chars();
            f.remove(pos..pos + l);
            f.insert(pos, converted.as_str());
            pos += l;
        }
        f.write_to(io::BufWriter::new(File::create(&self.path)?))?;
        Ok(())
    }
    pub fn translate(&self) -> io::Result<()> {
        let mut f = File::open(&self.path)?;
        let mut s = String::new();
        f.read_to_string(&mut s);
        let translator = TextTranslator::new();
        let converted = translator.translate(&s);
        let mut buff = io::BufWriter::new(File::create(&self.path)?);
        buff.write_all(converted.as_bytes())
    }
}

#[cfg(test)]
mod text_translation_tests {
    use super::TXTTranslator;

    // Tested with a file containing 100000 lines of ²³´µµ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüý¨··'°¯ª±£§¦«©®
    // Works in 27.8 seconds - is much slower
    #[test]
    fn test_with_ropey() {
        // Try with your files
        // let translator = TXTTranslator::new("");
        // assert!(translator.translate_with_ropey().is_ok());
    }

    // Works in 3.3 seconds
    #[test]
    fn test() {
        // Try with your files
        // let translator = TXTTranslator::new("");
        // assert!(translator.translate().is_ok());
    }
}
