mod translator;
use std::io;

pub struct Translator;

impl Translator {
    pub fn translate_text(text: &str) -> String {
        let translator = translator::text_translator::TextTranslator::new();
        translator.translate(text)
    }

    pub fn translate_txt(path: &str) -> io::Result<()> {
        let translator = translator::txt_translator::TXTTranslator::new(path);
        translator.translate()
    }

    pub fn translate_txt_from_stream<R: io::Read+io::Seek>(path: &mut R) -> io::Result<String> {
        translator::txt_translator::TXTTranslator::from_stream(path)
    }

    pub fn translate_docx(path: &str, output_path: &str) -> io::Result<()> {
        let translator = translator::docx_translator::DocXTranslator::new(path, output_path);
        translator.translate()
    }

    pub fn translate_docx_from_stream<R: io::Read+io::Seek>(path: &mut R) -> io::Result<String> {
        translator::docx_translator::DocXTranslator::from_stream(path)
    }

    pub fn translate_pptx(path: &str, output_path: &str) -> io::Result<()> {
        let translator = translator::pptx_translator::PPTXTranslator::new(path, output_path);
        translator.translate()
    }

    pub fn translate_pptx_from_stream<R: io::Read+io::Seek>(path: &mut R) -> io::Result<String> {
        translator::pptx_translator::PPTXTranslator::from_stream(path)
    }
}
