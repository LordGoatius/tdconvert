#![feature(trim_prefix_suffix)]
#![feature(panic_payload_as_str)]
use std::{env::args, panic::{self, PanicHookInfo}, process};

use ternary::{trits::Trit, word::Word};

fn main() {
    enum ConvertFrom {
        Decimal,
        Ternary,
    }

    // Useful error message without annoying panic information
    panic::set_hook(Box::new(|panic_hook_info: &PanicHookInfo| {
        println!("{}", panic_hook_info.payload_as_str().unwrap());
        process::exit(1);
    }));

    let help = "Please provide either a the --ternary (-t), to convert from ternary, or --decimal (-d) to convert from decimal";
    let args: Vec<String> = args().collect();
    let mode = match args.get(1).expect(help).as_str() {
        "--decimal" | "-d" => ConvertFrom::Decimal,
        "--ternary" | "-t" => ConvertFrom::Ternary,
        _ => panic!("{}", help),
    };

    let string = args
        .get(2)
        .expect("Please provide a ternary number to convert into decimal");

    match mode {
        ConvertFrom::Ternary => {
            let mut val: isize = 0;
            for (i, char) in string.chars().rev().enumerate() {
                val += match char {
                    '0' => 0,
                    '1' => 3isize.pow(i as u32),
                    'T' => -(3isize.pow(i as u32)),
                    _ => panic!("Invalid ternary digit. Please use '1', '0', or 'T'"),
                };
            }
            println!("{}", val);
        }
        ConvertFrom::Decimal => {
            let isize: isize = string
                .parse()
                .expect("Please provide a signed decimal number to convert into ternary");
            if isize > 3812798742493 {
                println!("The value has wrapped around the 27 trit maximum");
            }
            let ternary: Word = isize.into();
            let str = <Word as Into<[Trit; 27]>>::into(ternary)
                .map(<Trit as Into<char>>::into)
                .iter()
                .rev()
                .collect::<String>();
            let str = str.trim_start_matches('0');
            println!("{str}");
        }
    }
}
