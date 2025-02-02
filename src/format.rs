use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Notations {
    Scientific,
    HybridScientific,
    Standard,
    Engineering,
    None,
    Binary,
    Hex,
    Logarithm,
    Leaf,
    Emoji,
    Morse,
    Celeste,
    Heart,
    Reverse,
    Blind,
}

pub fn formatnum(notation: &Notations, input: f64) -> String {
    match notation {
        Notations::Scientific => scientific(input),
        Notations::HybridScientific => hybrid_scientific(input),
        Notations::Standard => standard(input),
        Notations::Engineering => engineering(input),
        Notations::None => none(input),
        Notations::Binary => binary(input),
        Notations::Hex => hex(input),
        Notations::Logarithm => logarithm(input),
        Notations::Emoji => emoji(input),
        Notations::Blind => blind(input),
        Notations::Morse => morse(input),
        Notations::Leaf => leaf(input),
        Notations::Reverse => reverse(input),
        Notations::Celeste => celeste(input),
        Notations::Heart => heart(input),
    }
}

fn scientific(input: f64) -> String {
    format!("{:.2e}", input)
}

fn hybrid_scientific(input: f64) -> String {
    if input < 10000.0 {
        format!("{:.2}", input)
    } else {
        format!("{:.2e}", input)
    }
}

fn standard(input: f64) -> String {
    // let abbreviation1 = ["","K","M","B","T","Qd","Qn","Sx","Sp","Oc","No"]; // only used once, use abbreviations 2 and 3 for everything above 1 No
    // let abbreviation2 = ["","U","D","T","Qa","Qn","Sx","Sp","Oc","No"];
    // let abbreviation3 = ["","De","Vg","Tg","Qd","Qn","Se","Sg","Og","Ng","Ce","Dn","Tc","Qe","Qu","Sc","Si","Oe","Ne"];

    // // 1dc = 1e33

    // if input < 1000.0 { // below 1K, dont abbreviate at all
    //     return input.to_string();
    // }

    // let truncated_str = input.trunc().to_string();
    // let exponent = input.log10().floor();
    // let closest_exponent = (exponent/3.0).floor();
    // let digits_to_display = (exponent % 3.0) as usize;
    // let mut number_to_display = truncated_str.chars().take(digits_to_display).collect::<String>();

    // if exponent < 33.0 { // below 1 Dc, use abbreviations 1
    //     number_to_display.push_str(abbreviation1[closest_exponent as usize]);
    // } else {
    //     number_to_display.push_str(abbreviation2[(closest_exponent % 11.0) as usize]);
    //     number_to_display.push_str(abbreviation3[(closest_exponent / 11.0).floor() as usize]);
    // }
    // format!("{}",number_to_display)
    format!("NOTIMPLEMENTED{:.2e}", input)
}

fn engineering(input: f64) -> String {
    let exponent = (input.log10().floor() as i32 / 3) * 3;
    let normalized_base = input / 10f64.powi(exponent);
    format!("{:.2}e{}", normalized_base, exponent)
}

fn none(input: f64) -> String {
    format!("{:.2}", input)
}

fn binary(input: f64) -> String {
    format!("{:064b}", input.to_bits())
}

fn hex(input: f64) -> String {
    format!("{:016x}", input.to_bits())
}

fn logarithm(input: f64) -> String {
    format!("e{}", input.log10())
}

fn leaf(input: f64) -> String {
    let abbreviations = [
        "", "k", "m", "b", "t", "a", "A", "c", "C", "d", "D", "e", "E", "f", "F", "g", "G", "h",
        "H", "i", "I", "j", "J", "n", "N", "o", "O", "p", "P", "q", "Q", "r", "R", "s", "S", "u",
        "U", "v", "V", "w", "W", "x", "X", "y", "Y", "z", "Z",
    ];

    // too big man
    if input > 10.0_f64.powf(144.0) {
        return format!("{:.3e}Z", input - 10.0_f64.powf(144.0));
    }

    let exponent = input.log10().floor();
    let closest_exponent: usize = (exponent / 3.0).floor() as usize;

    let scaled_number = input / 10_f64.powi((closest_exponent * 3) as i32);
    let formatted_number = format!("{:.3}", scaled_number);

    if exponent < 3.0 {
        //<1k, dont do anything
        return format!("{:.3}", input);
    }

    format!("{}{}", formatted_number, abbreviations[closest_exponent])
}

fn emoji(input: f64) -> String {
    let mut emojicodes = HashMap::new();
    emojicodes.insert('1', "🐦‍🔥");
    emojicodes.insert('2', "🍓");
    emojicodes.insert('3', "🔱");
    emojicodes.insert('4', "💅");
    emojicodes.insert('5', "🏳️‍⚧️");
    emojicodes.insert('6', "🎲");
    emojicodes.insert('7', "🎰");
    emojicodes.insert('8', "🎡");
    emojicodes.insert('9', "🫨");
    emojicodes.insert('0', "🕸️");
    emojicodes.insert('.', ".");

    let mut emojistring = String::new();
    for i in format!("{:.3}", input).chars() {
        if let Some(&emoji) = emojicodes.get(&i) {
            emojistring.push_str(emoji);
        }
    }
    emojistring
}

fn morse(input: f64) -> String {
    let morse_tuples = [
        ("A", ".-"),
        ("B", "-..."),
        ("C", "-.-."),
        ("D", "-.."),
        ("E", "."),
        ("F", "..-."),
        ("G", "--."),
        ("H", "...."),
        ("I", ".."),
        ("J", ".---"),
        ("K", "-.-"),
        ("L", ".-.."),
        ("M", "--"),
        ("N", "-."),
        ("O", "---"),
        ("P", ".--."),
        ("Q", "--.-"),
        ("R", ".-."),
        ("S", "..."),
        ("T", "-"),
        ("U", "..-"),
        ("V", "...-"),
        ("W", ".--"),
        ("X", "-..-"),
        ("Y", "-.--"),
        ("Z", "--.."),
        ("1", ".----"),
        ("2", "..---"),
        ("3", "...--"),
        ("4", "....-"),
        ("5", "....."),
        ("6", "-...."),
        ("7", "--..."),
        ("8", "---.."),
        ("9", "----."),
        ("0", "-----"),
    ];

    let base = morse_tuples.len() as f64;
    let mut morsestring = String::new();
    let mut quotient = input as f64;
    while quotient > 0.0 {
        let remainder = (quotient % base) as usize;
        morsestring.push_str(morse_tuples[remainder].0);
        quotient = (quotient / base).floor();
    }

    format!("{}", morsestring.chars().rev().collect::<String>())
}

fn celeste(input: f64) -> String {
    let mut celestecodes = HashMap::new();
    celestecodes.insert('1', ":maddyhug:");
    celestecodes.insert('2', ":baddyhug:");
    celestecodes.insert('3', ":lanihug:");
    celestecodes.insert('4', ":radgranny:");
    celestecodes.insert('5', ":theoretical:");
    celestecodes.insert('6', ":reaperline:");
    celestecodes.insert('7', ":fullclear:");
    celestecodes.insert('8', ":CrystalHeart:");
    celestecodes.insert('9', ":birb:");
    celestecodes.insert('0', ":catbus:");
    celestecodes.insert('.', ".");

    let mut celestestring = String::new();
    for i in format!("{:.3}", input).chars() {
        if let Some(&celeste) = celestecodes.get(&i) {
            celestestring.push_str(celeste);
        }
    }
    celestestring
}

fn heart(input: f64) -> String {
    let mut heartcodes = HashMap::new();
    heartcodes.insert('1', "❤");
    heartcodes.insert('2', "🧡");
    heartcodes.insert('3', "💛");
    heartcodes.insert('4', "💚");
    heartcodes.insert('5', "💙");
    heartcodes.insert('6', "💜");
    heartcodes.insert('7', "🤎");
    heartcodes.insert('8', "🖤");
    heartcodes.insert('9', "🤍");
    heartcodes.insert('0', "💔");
    heartcodes.insert('.', ".");

    let mut heartstring = String::new();
    for i in format!("{:.3}", input).chars() {
        if let Some(&heart) = heartcodes.get(&i) {
            heartstring.push_str(heart);
        }
    }
    heartstring // Return the constructed heart string
}

fn reverse(input: f64) -> String {
    format!(
        "{}",
        format!("{:.3}", input).chars().rev().collect::<String>()
    )
}

fn blind(_input: f64) -> String {
    format!("{}", "")
}
