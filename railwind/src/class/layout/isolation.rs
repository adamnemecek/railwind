use crate::class::{max_arg_count, min_arg_count, Decl};
use crate::ret_lit;
use crate::warning::WarningType;

pub fn parse_isolation(
    class_name: &str,
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Decl> {
    match class_name {
        "isolate" => {
            max_arg_count("isolate", args, 0, warnings);
            ret_lit!("isolation: isolate")
        }
        "isolation" => {
            if min_arg_count(args, 1, warnings) {
                match args[0] {
                    "auto" => {
                        max_arg_count("isolation", args, 1, warnings);
                        ret_lit!("isolation: auto")
                    }
                    _ => warnings.push(WarningType::InvalidArg(args[0].into(), vec!["auto"])),
                }
            }
        }
        _ => (),
    }

    None
}
