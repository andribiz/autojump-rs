use std::env;

use autojump::data;
use autojump::Config;

pub fn print_stat(config: &Config, with_weight: bool) {
    let cwd: Option<_> = match env::current_dir() {
        Ok(dir) => Some(dir),
        // The cwd is gone or inaccessible, disable weight reporting later.
        Err(_) => None,
    };
    let mut cwd_weight: Option<f64> = None;

    let entries = {
        let mut tmp = data::load(config);
        tmp.sort();
        tmp
    };
    let mut weight_sum = 0.0f64;
    for ref entry in &entries {
        if !with_weight {
            println!(
                "{}",
                entry.path.clone().into_os_string().into_string().unwrap()
            );
            continue;
        }

        println!("{}", entry);
        // NOTE: This isn't exactly accurate due to floating-point nature,
        // but since this is only an estimate let's get over it!
        weight_sum += entry.weight;

        // Simultaneously check for current directory's weight, if current
        // directory is accessible.
        if cwd.is_some() && cwd_weight.is_none() {
            if &entry.path == cwd.as_ref().unwrap() {
                cwd_weight = Some(entry.weight);
            }
        }
    }
    if !with_weight {
        return;
    }

    println!("⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯\n");
    println!("{:.0}:\t total weight", weight_sum.floor());
    println!("{}:\t number of entries", entries.len());

    if cwd.is_some() {
        let cwd_weight = cwd_weight.unwrap_or(0.0f64);
        println!("{:.2}:\t current directory weight", cwd_weight);
    }

    println!("\ndata:\t {}", config.data_path.to_string_lossy());
}
