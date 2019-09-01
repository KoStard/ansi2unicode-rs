mod translator;

#[cfg(test)]
mod tests {
    use super::translator;

    #[test]
    fn it_works() {
        let t = translator::text_translator::TextTranslator::new();
        assert_eq!(t.translate("²³´µµ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüý¨··'°¯ª±£§¦«©®"), String::from("ԱաԲբբԳգԴդԵեԶզԷէԸըԹթԺժԻիԼլԽխԾծԿկՀհՁձՂղՃճՄմՅյՆնՇշՈոՉչՊպՋջՌռՍսՎվՏտՐրՑցՒւՓփՔքՕօՖֆևգգ՚՛՜՝՞։«»,.…"));
    }
}
