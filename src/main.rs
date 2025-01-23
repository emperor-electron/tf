use chrono::Utc;
use clap::Parser;
use colored::*;
use std::{error::Error, fs, os::unix::fs::PermissionsExt, process};

/// Utility for generating files in supported file types
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of file to be generated
    #[arg(required_if_eq("supported_filetypes", "false"))]
    name: Option<String>,

    /// List of supported filetypes
    #[arg(short, long, default_value_t = false)]
    supported_filetypes: bool,
}

#[derive(Debug, PartialEq)]
enum FileTypes {
    C,
    H,
    Python,
    CPP,
    HPP,
    Bash,
    SystemVerilogModule,
    SystemVerilogPackage,
}

#[derive(Debug)]
struct Info {
    date: String,
    author: String,
    file: String,
}

fn create_file(filename: &str, filetype: FileTypes) -> Result<(), Box<dyn Error>> {
    let now = Utc::now();
    let date = now.format("%m/%d/%Y").to_string();

    let mut info = Info {
        date: date.clone(),
        file: filename.to_string(),
        author: env!("LOGNAME", "$LOGNAME isn't defined?").to_string(),
    };

    match filetype {
        FileTypes::C => {
            let filename_string = format!("{filename}.c");
            info.file = filename_string;
            fs::write(&info.file, create_c_file(&info))?;
        }
        FileTypes::H => {
            let filename_string = format!("{filename}.h");
            info.file = filename_string;
            fs::write(&info.file, create_h_file(&info))?;
        }
        FileTypes::Python => {
            let filename_string = format!("{filename}.py");
            info.file = filename_string;
            fs::write(&info.file, create_py_file(&info))?;
        }
        FileTypes::CPP => {
            let filename_string = format!("{filename}.cpp");
            info.file = filename_string;
            fs::write(&info.file, create_cpp_file(&info))?;
        }
        FileTypes::HPP => {
            let filename_string = format!("{filename}.hpp");
            info.file = filename_string;
            fs::write(&info.file, create_hpp_file(&info))?;
        }
        FileTypes::Bash => {
            let filename_string = format!("{filename}.bash");
            info.file = filename_string;
            fs::write(&info.file, create_bash_file(&info))?;
            let mut perms = fs::metadata(&info.file)?.permissions();
            perms.set_mode(0o744);
            fs::set_permissions(&info.file, perms)?;
        }
        FileTypes::SystemVerilogModule => {
            let filename_string = format!("{filename}.sv");
            info.file = filename_string;
            fs::write(&info.file, create_sv_file(&info))?;
        }
        FileTypes::SystemVerilogPackage => {
            let filename_string = format!("{filename}.svh");
            info.file = filename_string;
            fs::write(&info.file, create_svh_file(&info))?;
        }
    }

    Ok(())
}

fn check_input_errs(input: &Vec<&str>) -> Result<(), String> {
    if *input == [""] {
        return Err(String::from("Input filename is expected."));
    }
    if input.len() <= 1 {
        return Err(String::from("Filename with file extension is expected."));
    }

    Ok(())
}

fn show_supported_filetypes() {
    println!("{}", "Software Filetypes:".bright_cyan().bold().underline());
    println!(
        "  {}      : '{}'",
        "C".bright_cyan().bold(),
        ".c".bright_green().bold()
    );
    println!(
        "  {}      : '{}'",
        "H".bright_cyan().bold(),
        ".h".bright_green().bold()
    );
    println!(
        "  {} : '{}'",
        "Python".bright_cyan().bold(),
        ".py".bright_green().bold()
    );
    println!(
        "  {}    : '{}'",
        "CPP".bright_cyan().bold(),
        ".cpp".bright_green().bold()
    );
    println!(
        "  {}    : '{}'",
        "HPP".bright_cyan().bold(),
        ".hpp".bright_green().bold()
    );
    println!(
        "  {}   : '{}'",
        "Bash".bright_cyan().bold(),
        ".bash".bright_green().bold()
    );
    println!("");
    println!("{}", "HDL Filetypes:".bright_cyan().bold().underline());
    println!(
        "  {}  : '{}'",
        "SystemVerilog (module)".bright_cyan().bold(),
        ".sv".bright_green().bold()
    );
    println!(
        "  {} : '{}'",
        "SystemVerilog (package)".bright_cyan().bold(),
        ".svh".bright_green().bold()
    );
    process::exit(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.supported_filetypes {
        show_supported_filetypes()
    }

    let input_filename = args.name.unwrap_or_else(|| {
        eprintln!(
            "{}: Program requires argument. See help with 'tf --help'",
            "ERROR".red()
        );
        process::exit(1)
    });

    let filename: Vec<&str> = input_filename.split(".").collect();

    if let Err(msg) = check_input_errs(&filename) {
        eprintln!("{} with input: {msg}", "ERROR".red());
        process::exit(1);
    }

    let filetype: FileTypes = match filename.last() {
        Some(&"c") => FileTypes::C,
        Some(&"h") => FileTypes::H,
        Some(&"py") => FileTypes::Python,
        Some(&"cpp") => FileTypes::CPP,
        Some(&"hpp") => FileTypes::HPP,
        Some(&"bash") => FileTypes::Bash,
        Some(&"sv") => FileTypes::SystemVerilogModule,
        Some(&"svh") => FileTypes::SystemVerilogPackage,
        Some(&unsupported_filetype) => {
            eprintln!("{}: Filetype '.{unsupported_filetype}' is not supported. Run 'tf --list-filetypes' for available filetypes.", "ERROR".red());
            process::exit(1)
        }
        None => {
            panic!("Why are you the way that you are? :(");
        }
    };

    if let Err(e) = create_file(filename.first().unwrap(), filetype) {
        eprintln!("{} creating file: {e}", "ERROR".red());
        process::exit(1);
    };

    Ok(())
}

fn create_c_file(info: &Info) -> String {
    String::from(format!(
        "////////////////////////////////////////////////////////////////////////
// Author  : {}
// File    : {}
// Date    : {}
// Purpose : TODO
////////////////////////////////////////////////////////////////////////

#include <stdio.h>

int main(int argc, char *argv[]) {{
  printf(\"Hello, World!\\n\");
  return 0;
}}

",
        info.author, info.file, info.date,
    ))
}

fn create_h_file(info: &Info) -> String {
    let guard = info.file.replace(".", "_").to_uppercase();
    String::from(format!(
        "////////////////////////////////////////////////////////////////////////
// Author  : {}
// File    : {}
// Date    : {}
// Purpose : TODO
////////////////////////////////////////////////////////////////////////

#ifndef {guard}
#define {guard}

// STRUCTS

// FUNCTIONS

////////////////////////////////////////////////////////////////////////
#endif
",
        info.author, info.file, info.date,
    ))
}

fn create_py_file(info: &Info) -> String {
    String::from(format!(
        "\"\"\"
Author  : {}
File    : {}
Date    : {}
Purpose : TODO
\"\"\"


def main() -> int:
    return 0


if __name__ == \"__main__\":
    main()",
        info.author, info.file, info.date,
    ))
}

fn create_cpp_file(info: &Info) -> String {
    String::from(format!(
        "////////////////////////////////////////////////////////////////////////
// Author  : {}
// File    : {}
// Date    : {}
// Purpose : TODO
////////////////////////////////////////////////////////////////////////

#include <iostream>

int main(int argc, char *argv[]) {{
  std::cout << \"Hello, World!\" << std::endl;
  return 0;
}}

",
        info.author, info.file, info.date,
    ))
}

fn create_hpp_file(info: &Info) -> String {
    String::from(format!(
        "////////////////////////////////////////////////////////////////////////
// Author  : {}
// File    : {}
// Date    : {}
// Purpose : TODO
////////////////////////////////////////////////////////////////////////

#pragma once

// STRUCTS

// FUNCTIONS

////////////////////////////////////////////////////////////////////////
",
        info.author, info.file, info.date,
    ))
}

fn create_bash_file(info: &Info) -> String {
    String::from(format!(
        "#!/bin/bash
########################################################################
# Author  : {}
# File    : {}
# Date    : {}
# Purpose : TODO
########################################################################
set -e # exit immediately on error
set -u # treat unbound variables as errors
set -x # enable tracing

echo \"Hello, World!\"
",
        info.author, info.file, info.date,
    ))
}

fn create_sv_file(info: &Info) -> String {
    let module_name: Vec<&str> = info.file.split(".").collect();

    String::from(format!(
        "////////////////////////////////////////////////////////////////////////
// Author  : {}
// File    : {}
// Date    : {}
// Purpose : TODO
////////////////////////////////////////////////////////////////////////

`default_nettype none

module {} (
  input logic clk,
  input logic rst
  );

  // TODO - Implementation

endmodule

`default_nettype wire

",
        info.author, info.file, info.date, module_name[0]
    ))
}

fn create_svh_file(info: &Info) -> String {
    let package_name: Vec<&str> = info.file.split(".").collect();
    let package_name_no_file_ext = package_name[0];
    let header_guard = package_name_no_file_ext.to_uppercase();

    String::from(format!(
        "////////////////////////////////////////////////////////////////////////
// Author  : {}
// File    : {}
// Date    : {}
// Purpose : TODO
////////////////////////////////////////////////////////////////////////

`ifndef {}
`define {}

package {};

  // TODO - Implementation

endpackage: {}

`endif

",
        info.author,
        info.file,
        info.date,
        header_guard,
        header_guard,
        package_name_no_file_ext,
        package_name_no_file_ext
    ))
}
