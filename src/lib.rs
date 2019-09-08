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

    pub fn translate_docx(path: &str, output_path: &str) -> io::Result<()> {
        let translator = translator::docx_translator::DocXTranslator::new(path, output_path);
        translator.translate()
    }

    pub fn translate_pptx(path: &str, output_path: &str) -> io::Result<()> {
        let translator = translator::pptx_translator::PPTXTranslator::new(path, output_path);
        translator.translate()
    }
}
