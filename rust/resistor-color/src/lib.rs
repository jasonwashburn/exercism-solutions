use int_enum::IntEnum;
use enum_iterator::{all,Sequence};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange= 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey= 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value().into()
}

pub fn value_to_color_string(value: u32) -> String {
    let resistor_enum = ResistorColor::from_int(value.try_into().unwrap());
    let color = match resistor_enum {
        Ok(ResistorColor::Black) => String::from("Black"),
        Ok(ResistorColor::Brown) => String::from("Brown"),
        Ok(ResistorColor::Red) => String::from("Red"),
        Ok(ResistorColor::Orange) => String::from("Orange"),
        Ok(ResistorColor::Yellow) => String::from("Yellow"),
        Ok(ResistorColor::Green) => String::from("Green"),
        Ok(ResistorColor::Blue) => String::from("Blue"),
        Ok(ResistorColor::Violet) => String::from("Violet"),
        Ok(ResistorColor::Grey) => String::from("Grey"),
        Ok(ResistorColor::White) => String::from("White"),
        Err(_) => String::from("value out of range"),
    };
    return color;
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
