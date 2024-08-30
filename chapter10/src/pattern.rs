/// リテラル
pub const HELLO: &'static str = "Hello, world!";

#[cfg(test)]
mod test {
    use super::HELLO;
    #[test]
    fn match_literal() {
        let s = "Hello, world!";
        let matched = match s {
            "hello" => "Hello back!",
            HELLO => {
                let message = format!("{} from Japan", HELLO);
                Box::leak(Box::new(message))
            }
            _ => "I don't know what you said",
        };
        assert_eq!(matched, "Hello, world! from Japan");
    }

    #[test]
    fn test_range_match() {
        let x = 1;
        let matched = match x {
            1..=5 => "one through five",
            _ => "something else",
        };
        assert_eq!(matched, "one through five");
    }

    #[test]
    fn test_ref_valiable_valiation() {
        let x = 1;
        let matched = match x {
            ref r => r,
        };
        assert_eq!(*matched, 1);
    }

    #[test]
    fn test_enum_subpattern_hold_valiable() {
        #[derive(Debug)]
        enum Message {
            Hello { id: i32 },
        }
        let msg = Message::Hello { id: 5 };
        let matched: &i32 = match msg {
            Message::Hello { ref id } => id,
        };
        assert_eq!(*matched, 5);
    }

    #[test]
    fn test_enum_match() {
        #[derive(Debug)]
        enum Message {
            Hello { id: i32 },
            Bye,
        }
        let msg = Message::Hello { id: 5 };
        let matched = match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => id_variable,
            Message::Hello { id } => id,
            Message::Bye => 0,
        };
        assert_eq!(matched, 5);
    }

    #[test]
    fn test_enum_match_guard() {
        #[derive(Debug)]
        enum Message {
            Hello { id: i32 },
            Bye,
        }
        let msg = Message::Hello { id: 5 };
        let matched = match msg {
            Message::Hello { id } if id > 5 => "greater than five",
            Message::Hello { id } if id < 5 => "less than five",
            Message::Hello { id } => "five",
            Message::Bye => "bye",
        };
        assert_eq!(matched, "five");
    }

    #[test]
    fn test_emun_match_binding() {
        #[derive(Debug)]
        enum Message {
            Hello { id: i32 },
            Bye,
        }
        let msg = Message::Hello { id: 5 };
        let matched = match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => id_variable,
            Message::Hello { id } => id,
            Message::Bye => 0,
        };
        assert_eq!(matched, 5);
    }

    #[test]
    fn test_enum_three_tuple_match() {
        #[derive(Debug)]
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        let color = Color::Rgb(122, 17, 40);
        let matched = match color {
            Color::Rgb(r, g, b) => (r, g, b),
            Color::Hsv(h, s, v) => (h, s, v),
        };
        assert_eq!(matched, (122, 17, 40));
    }

    #[test]
    fn test_enum_struct_match() {
        #[derive(Debug)]
        enum Message {
            Hello { id: i32 },
            Bye,
        }
        let msg = Message::Hello { id: 5 };
        let matched = match msg {
            Message::Hello { id } => id,
            Message::Bye => 0,
        };
        assert_eq!(matched, 5);
    }

    #[test]
    fn test_ref_match() {
        let x = &5;
        let matched = match x {
            &val => val,
        };
        assert_eq!(matched, 5);
    }

    #[test]
    fn test_enum_or_match() {
        #[derive(Debug)]
        enum Message {
            Hello { id: i32 },
            Hey { id: i32 },
            Bye,
        }
        let msg = Message::Hello { id: 5 };
        let matched = match msg {
            Message::Hello { id } | Message::Hey { id } => id,
            Message::Bye => 0,
        };
        assert_eq!(matched, 5);
    }

    #[test]
    fn test_guard_match() {
        let x = 4;
        let matched = match x {
            val if val < 5 => "less than five",
            val if val == 5 => "five",
            val if val > 5 => "greater than five",
            _ => "something else",
        };
        assert_eq!(matched, "less than five");
    }
}
