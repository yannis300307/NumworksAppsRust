use std::process::Command;

fn compile_c_libs() {
    unsafe { std::env::set_var("CC", "arm-none-eabi-gcc") };

    let program = "npx";

    let nwlink_flags = String::from_utf8(
        Command::new(program)
            .args(["--yes", "--", "nwlink@0.0.19", "eadk-cflags"])
            .output()
            .expect("Failed to get nwlink eadk-cflags")
            .stdout,
    )
    .expect("Invalid UTF-8 in nwlink flags");

    let mut build = cc::Build::new();
    build.file("src/eadk/storage/storage.c");
    build.flag("-std=c99");
    build.flag("-Os");
    build.flag("-Wall");
    build.flag("-ggdb");
    build.warnings(false);

    for flag in nwlink_flags.split_whitespace() {
        build.flag(flag);
    }

    build.compile("storage_c");
}

fn convert_icon() {
    let output = {
        if let Ok(out) = Command::new("sh")
            .arg("-c")
            .arg("npx --yes -- nwlink@0.0.19 png-nwi assets/icon.png target/icon.nwi")
            .output()
        {
            out
        } else {
            panic!(
                "Your OS is not supported! If you're using Windows, please compile Numcraft in WSL."
            );
        }
    };
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn main() {
    // Turn icon.png into icon.nwi
    println!("cargo:rerun-if-changed=assets/icon.png");
    convert_icon();

    // Compile storage.c
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "none" {
        compile_c_libs();
    }
}
