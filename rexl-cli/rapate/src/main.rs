use crate::apate::{Apate, DisguiseType};
use rexl::argparse::FromArgs;
use rexl::text::to_size_str;
use std::env;
use std::process;

mod apate;
mod convert;

#[derive(Debug, FromArgs)]
pub struct Main {
    #[arg_parser(name = "i,f,input,file")]
    pub input_file: String,
    pub decode: bool,
    pub jpg: bool,
    pub mov: bool,
    pub mp4: bool,
    pub exe: bool,
    #[arg_parser(name = "m,mask")]
    pub mask_file: String,
    #[arg_parser(name = "y")]
    pub yes: bool,
}

impl From<Main> for DisguiseType {
    fn from(m: Main) -> Self {
        if m.jpg {
            Self::Jpg
        } else if m.mov {
            Self::Mov
        } else if m.mp4 {
            Self::Mp4
        } else if m.exe {
            Self::Exe
        } else if !m.mask_file.is_empty() {
            Self::MaskFile(m.mask_file)
        } else {
            Self::Mp4 // prefer mp4
        }
    }
}

pub fn main() {
    let args = env::args().skip(1).collect();

    let main = match Main::from_args(args) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let decode = main.decode;
    let input_file = main.input_file.clone();
    let dry_run = !main.yes;
    if dry_run {
        println!("选择了dry-run模式，不实际修改文件")
    } else {
        println!("选择了非dry-run模式，将实际原地修改文件")
    }

    let apate = Apate { disguise_type: main.into(), dry_run };
    let res = if decode { apate.reveal(&input_file) } else { apate.disguise(&input_file) };

    let desc = if decode { "文件还原" } else { "文件伪装" };
    match res {
        Ok(size) => {
            println!("{}成功，size={}", desc, to_size_str(size));
        }
        Err(err) => {
            eprintln!("{}失败：{}", desc, err);
            process::exit(1);
        }
    };
}
