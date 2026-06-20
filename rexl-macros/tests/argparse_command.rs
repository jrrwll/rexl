use rexl::argparse::{ArgParserRunnable, FromArgs, RunWithArgs, run_with_args_tree};
use serde::Serialize;

#[derive(Debug, Serialize, FromArgs)]
struct App {
    help: bool,
    verbose: Option<bool>,
}

#[derive(Debug, Serialize, FromArgs)]
struct AppList {
    user: String,
    page_size: u32,
    page_num: Option<usize>,
}

#[derive(Debug, Serialize, FromArgs)]
#[arg_parser(first_char)]
struct AppDelete {
    // value
    user: String,
    force: bool,
}

#[derive(Debug, Serialize, FromArgs)]
#[arg_parser(first_char)]
struct AppCreate {
    help: bool,
}

#[derive(Debug, Serialize, FromArgs)]
#[arg_parser(first_char)]
struct AppCreateImage {
    // value
    name: String,
}

#[derive(Debug, Serialize, FromArgs)]
#[arg_parser(first_char)]
struct AppCreateVolume {
    name: Option<String>,
    ratio: f32,
}

run_with_args_tree! {
    App {
        "l,list" => AppList,
        "delete" => AppDelete,
        "c,n,create,new" => AppCreate {
            "i,image" => AppCreateImage,
            "v,volume" => AppCreateVolume,
        },
    }
}

macro_rules! impl_runnable {
    ($my_type:ty) => {
        impl ArgParserRunnable for $my_type {
            fn run(self) {
                let type_name = std::any::type_name::<$my_type>();
                let s = serde_json::to_string_pretty(&self).expect("failed to serialize");
                println!("entrypoint on {}: {}", type_name, s);
            }
        }
    };
}

impl_runnable!(App);
impl_runnable!(AppList);
impl_runnable!(AppDelete);
impl_runnable!(AppCreate);
impl_runnable!(AppCreateImage);
impl_runnable!(AppCreateVolume);

// cargo expand --test argparse_command
#[test]
fn test_app() {
    let args_list = vec![
        vec!["--help"],
        vec!["l", "--user", "me"],
        vec!["list", "--page-size", "20"],
        vec!["delete", "-u", "me", "--force"],
        vec!["create", "-h"],
        vec!["create", "image", "-n", "my-image"],
        vec!["create", "volume", "--ratio", "0.5"],
    ];
    for args in args_list {
        let args = to_vec_string(args);
        App::run_with_args(args).expect("failed to parse args");
    }
}

fn to_vec_string(args: Vec<&str>) -> Vec<String> {
    args.into_iter().map(|s| s.to_string()).collect()
}
