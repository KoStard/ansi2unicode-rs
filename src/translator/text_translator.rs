use std::collections::HashMap;
use std::str;

pub struct TextTranslator;

impl TextTranslator {
    pub fn new() -> Self {
        TextTranslator
    }
}

impl TextTranslator {
    pub fn translate(&self, inp: &str) -> String {
        Self::translate_from_chars(inp.chars())
    }
    pub fn translate_from_chars(inp: str::Chars) -> String {
        let map: HashMap<char, char> = [
            ('²', 'Ա'),
            ('³', 'ա'),
            ('´', 'Բ'),
            ('µ', 'բ'),
            ('μ', 'բ'),
            ('¶', 'Գ'),
            ('·', 'գ'),
            ('¸', 'Դ'),
            ('¹', 'դ'),
            ('º', 'Ե'),
            ('»', 'ե'),
            ('¼', 'Զ'),
            ('½', 'զ'),
            ('¾', 'Է'),
            ('¿', 'է'),
            ('À', 'Ը'),
            ('Á', 'ը'),
            ('Â', 'Թ'),
            ('Ã', 'թ'),
            ('Ä', 'Ժ'),
            ('Å', 'ժ'),
            ('Æ', 'Ի'),
            ('Ç', 'ի'),
            ('È', 'Լ'),
            ('É', 'լ'),
            ('Ê', 'Խ'),
            ('Ë', 'խ'),
            ('Ì', 'Ծ'),
            ('Í', 'ծ'),
            ('Î', 'Կ'),
            ('Ï', 'կ'),
            ('Ð', 'Հ'),
            ('Ñ', 'հ'),
            ('Ò', 'Ձ'),
            ('Ó', 'ձ'),
            ('Ô', 'Ղ'),
            ('Õ', 'ղ'),
            ('Ö', 'Ճ'),
            ('×', 'ճ'),
            ('Ø', 'Մ'),
            ('Ù', 'մ'),
            ('Ú', 'Յ'),
            ('Û', 'յ'),
            ('Ü', 'Ն'),
            ('Ý', 'ն'),
            ('Þ', 'Շ'),
            ('ß', 'շ'),
            ('à', 'Ո'),
            ('á', 'ո'),
            ('â', 'Չ'),
            ('ã', 'չ'),
            ('ä', 'Պ'),
            ('å', 'պ'),
            ('æ', 'Ջ'),
            ('ç', 'ջ'),
            ('è', 'Ռ'),
            ('é', 'ռ'),
            ('ê', 'Ս'),
            ('ë', 'ս'),
            ('ì', 'Վ'),
            ('í', 'վ'),
            ('î', 'Տ'),
            ('ï', 'տ'),
            ('ð', 'Ր'),
            ('ñ', 'ր'),
            ('ò', 'Ց'),
            ('ó', 'ց'),
            ('ô', 'Ւ'),
            ('õ', 'ւ'),
            ('ö', 'Փ'),
            ('÷', 'փ'),
            ('ø', 'Ք'),
            ('ù', 'ք'),
            ('ú', 'Օ'),
            ('û', 'օ'),
            ('ü', 'Ֆ'),
            ('ý', 'ֆ'),
            ('¨', 'և'),
            ('•', 'գ'),
            ('\'', '՚'),
            ('°', '՛'),
            ('¯', '՜'),
            ('ª', '՝'),
            ('±', '՞'),
            ('£', '։'),
            ('§', '«'),
            ('¦', '»'),
            ('«', ','),
            ('©', '.'),
            ('®', '…'),
        ]
        .iter()
        .cloned()
        .collect();
        inp.map(|c| *map.get(&c).unwrap_or(&c))
            .collect()
    }
}

#[cfg(test)]
mod text_translation_tests {
    use crate::translator::text_translator::TextTranslator;

    #[test]
    fn test_text() {
        let t = TextTranslator::new();
        assert_eq!(t.translate("²³´µµ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüý¨··'°¯ª±£§¦«©®"), String::from("ԱաԲբբԳգԴդԵեԶզԷէԸըԹթԺժԻիԼլԽխԾծԿկՀհՁձՂղՃճՄմՅյՆնՇշՈոՉչՊպՋջՌռՍսՎվՏտՐրՑցՒւՓփՔքՕօՖֆևգգ՚՛՜՝՞։«»,.…"));
    }
}
