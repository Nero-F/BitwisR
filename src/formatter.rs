type ResultLine = (char, isize);

pub fn get_power_of_two(value: isize) -> u8 {
    let mut p: u8 = 8;
    let v: isize = if value < 0 { value * -1 } else { value };

    loop {
        if isize::pow(2, p.into()) >= v {
            break;
        }
        p += 8;
    }
    if value <= -255 {
        p += 8;
    }
    p
}

pub fn format_signed_output2(token: char, v: isize, width: usize) -> String {
    if v < 0 {
        format!(
            "{} -{:width$}  {} -{:#x}",
            token,
            -1 * v,
            format_bin_string(v),
            -1 * v,
            width = width
        )
    } else {
        format!(
            "{}  {:width$}  {}  {:#x}",
            token,
            v,
            format_bin_string(v),
            v,
            width = width
        )
    }
}

pub fn get_max_and_index(tab: Vec<isize>) -> (isize, usize) {
    let mut max_index = 0;
    let mut max = tab[0];

    for (index, &x) in tab.iter().enumerate() {
        if x > max {
            max = x;
            max_index = index;
        }
    }
    //if max == 0 {
    //max = 1;
    //max_index = 2;
    //}
    (max, max_index)
}

pub fn align_values(values: ResultLine, width: usize) -> String {
    format_signed_output2(values.0, values.1, width)
}

pub fn format_bin_string(value: isize) -> String {
    let mut buffer: String;
    let mut power = get_power_of_two(value);
    let power_ref = f64::log2(value as f64);

    if (power as f64) == power_ref {
        power += 8;
    }
    if value < 0 {
        buffer = format!("{:3b}", value as i32);
        buffer = buffer[32 - power as usize..].to_string();
    } else {
        buffer = format!("{:0power$b}", value, power = power.into());
    }

    buffer
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 8 == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

pub fn format_signed_output(token: &str, v: isize) -> String {
    if v < 0 {
        format!(
            "{} -{}  {} -{:#x}",
            token,
            -1 * v,
            format_bin_string(v),
            -1 * v
        )
    } else {
        format!("{}  {}  {}  {:#x}", token, v, format_bin_string(v), v)
    }
}

pub fn format_hex_signed_output(token: &str, h: &str) -> String {
    let hl = h.to_lowercase();
    let mut neg = false;

    let trimmed = if h.chars().nth(0).unwrap() == '-' {
        neg = true;
        hl.trim_start_matches("-0x")
    } else {
        hl.trim_start_matches("0x")
    };
    let v = isize::from_str_radix(trimmed, 16).unwrap();

    if neg == true {
        format!("{} {} {} {}", token, hl, format_bin_string(-1 * v), -1 * v)
    } else {
        format!("{}  {} {}  {}", token, hl, format_bin_string(v), v)
    }
}

#[cfg(test)]
mod u_tests {
    use super::*;

    #[test]
    fn test_format_bin_string() {
        let value = 1;
        let value2 = 255;
        let value3 = 256;
        let value4 = 65535;
        let value5 = 65536;
        let value6 = 16777215;
        let value7 = 16777216;
        let value8 = 2147483647;
        let value9 = 2147483648;
        let value10: isize = 2147483649;
        let value11: isize = 4294967295; // this should be the max we can display

        assert_eq!("00000001", format_bin_string(value));
        assert_eq!("11111111", format_bin_string(value2));
        assert_eq!("00000001 00000000", format_bin_string(value3));
        assert_eq!("11111111 11111111", format_bin_string(value4));
        assert_eq!("00000001 00000000 00000000", format_bin_string(value5));
        assert_eq!("11111111 11111111 11111111", format_bin_string(value6));
        assert_eq!(
            "00000001 00000000 00000000 00000000",
            format_bin_string(value7)
        );
        assert_eq!(
            "01111111 11111111 11111111 11111111",
            format_bin_string(value8)
        );
        assert_eq!(
            "10000000 00000000 00000000 00000000",
            format_bin_string(value9)
        );
        assert_eq!(
            "10000000 00000000 00000000 00000001",
            format_bin_string(value10)
        );
        assert_eq!(
            "11111111 11111111 11111111 11111111",
            format_bin_string(value11)
        );
    }

    #[test]
    fn test_format_negative_bin_string() {
        let value = -1;
        let value2 = -255;
        let value3 = -256;
        let value4 = -65535;
        let value5 = -65536;
        let value6 = -16777215;

        assert_eq!("11111111", format_bin_string(value));
        assert_eq!("11111111 00000001", format_bin_string(value2));
        assert_eq!("11111111 00000000", format_bin_string(value3));
        assert_eq!("11111111 00000000 00000001", format_bin_string(value4));
        assert_eq!("11111111 00000000 00000000", format_bin_string(value5));
        assert_eq!(
            "11111111 00000000 00000000 00000001",
            format_bin_string(value6)
        );
    }
}
