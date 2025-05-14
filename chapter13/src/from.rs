

pub fn string_into(str: &String) -> Vec<u8> {

    let fmt_str = format!("{:?}", str);
    let into_str: Vec<u8> = fmt_str.into();
    into_str
}