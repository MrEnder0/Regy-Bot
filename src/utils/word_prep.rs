pub fn filter_characters(input: &str) -> String {
    let mut output = String::new();
    for character in input.chars() {
        match character {
            'а' | 'ạ' | 'ą' | 'ä' | 'à' | 'á' => output.push_str("a"),
            'ß' => output.push_str("b"),
            'с' | 'ç' | 'ć' | 'č' => output.push_str("c"),
            'ԁ' | 'đ' | 'ɗ' => output.push_str("d"),
            'е' | '€' | 'ę' | 'ë' | 'è' | 'é' | 'ẹ' | 'ẽ' | 'ẻ' => output.push_str("e"),
            'ƒ' => output.push_str("f"),
            'ğ' | 'ġ' | 'ģ' => output.push_str("g"),
            'һ' | 'ħ' => output.push_str("h"),
            'ӏ' | 'і' | 'ï' | 'ı' | 'ì' | 'í' | 'ị' | 'į' | 'ĩ' | 'ỉ' => output.push_str("i"),
            'ĵ' => output.push_str("j"),
            'ķ' | 'κ' => output.push_str("k"),
            'ł' | 'ḷ' => output.push_str("l"),
            'ṃ' | 'ṁ' => output.push_str("m"),
            'ո' | 'ñ' | 'ń' | 'ņ' | 'ň' | 'ŉ' => output.push_str("n"),
            'о' | 'ο' | 'օ' | 'ö' | 'ø' | 'õ' | 'ò' | 'ó' => output.push_str("o"),
            'р' | 'ṗ' => output.push_str("p"),
            'զ' => output.push_str("q"),
            'ř' | 'ŕ' | 'ŗ' => output.push_str("r"),
            'ʂ' | 'š' | 'ş' | 'ś' | 'ṣ' | 'ṡ' => output.push_str("s"),
            'ţ' | 'ť' | 'ṭ' | 'ṫ' => output.push_str("t"),
            'ս' | 'υ' | 'ü' | 'ù' | 'ú' | 'ụ' | 'ų' | 'ū' | 'ũ' | 'ủ' | 'ư' => output.push_str("u"),
            'ν' | 'ѵ' | 'ṿ' => output.push_str("v"),
            'ẁ' | 'ẃ' | 'ẅ' | 'ẇ' | 'ẉ' | 'ẘ' => output.push_str("w"),
            'х' | 'ҳ' | 'ẋ' | 'ẍ' => output.push_str("x"),
            'у' | 'ÿ' | 'ỳ' | 'ý' | 'ỵ' | 'ỹ' | 'ỷ' => output.push_str("y"),
            'ʐ' | 'ž' | 'ż' | 'ź' => output.push_str("z"),
            _ => output.push(character),
        }
    }
    output
}
