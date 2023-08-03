pub fn filter_characters(input: &str) -> String {
    let mut output = String::new();
    for character in input.chars() {
        match character {
            '@' | 'а' | 'ạ' | 'ą' | 'ä' | 'à' | 'á' => output.push_str("a"),
            'ß' => output.push_str("b"),
            'ç' | 'ć' | 'č' => output.push_str("c"),
            'đ' => output.push_str("d"),
            '€' | 'ę' | 'ë' | 'è' | 'é' | 'ẹ' | 'ẽ' | 'ẻ' => output.push_str("e"),
            'ƒ' => output.push_str("f"),
            'ğ' | 'ġ' | 'ģ' => output.push_str("g"),
            'ħ' => output.push_str("h"),
            'ï' | 'ı' | 'ì' | 'í' | 'ị' | 'į' | 'ĩ' | 'ỉ' => output.push_str("i"),
            'ĵ' => output.push_str("j"),
            'ķ' => output.push_str("k"),
            'ł' | '1' => output.push_str("l"),
            'ṃ' | 'ṁ' => output.push_str("m"),
            'ñ' | 'ń' | 'ņ' | 'ň' | 'ŉ' => output.push_str("n"),
            'ö' | 'ø' | 'õ' | 'ò' | 'ó' | '0' => output.push_str("o"),
            'ṗ' => output.push_str("p"),
            '9' => output.push_str("q"),
            'ř' | 'ŕ' | 'ŗ' => output.push_str("r"),
            'š' | 'ş' | 'ś' | 'ṣ' | 'ṡ' => output.push_str("s"),
            'ţ' | 'ť' | 'ṭ' | 'ṫ' => output.push_str("t"),
            'ü' | 'ù' | 'ú' | 'ụ' | 'ų' | 'ū' | 'ũ' | 'ủ' | 'ư' => output.push_str("u"),
            'ṿ' => output.push_str("v"),
            'ẁ' | 'ẃ' | 'ẅ' | 'ẇ' | 'ẉ' | 'ẘ' => output.push_str("w"),
            'ẋ' | 'ẍ' => output.push_str("x"),
            'ÿ' | 'ỳ' | 'ý' | 'ỵ' | 'ỹ' | 'ỷ' => output.push_str("y"),
            'ž' | 'ż' | 'ź' => output.push_str("z"),
            _ => output.push(character),
        }
    }
    output
}
