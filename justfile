app_name := "SampleApp"
lib_file_name := "libsample_app"

current_target := shell("rustc -vV | grep \"host:\" | awk '{print $2}'")

build:
    cargo build --release --bin {{app_name}} --target=thumbv7em-none-eabihf

send:
    cargo run --release --bin {{app_name}} --target=thumbv7em-none-eabihf

check:
    cargo build --release --bin {{app_name}} --target=thumbv7em-none-eabihf
    cargo build --release --target={{current_target}} --lib

[macos]
run_nwb:
    ./simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/release/{{lib_file_name}}.dylib

[linux]
run_nwb:
    ./simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/release/{{lib_file_name}}.so

sim jobs="1" features="":
    if [ ! -f "./simulator/output/release/simulator/linux/epsilon.bin" ]; then \
        cd simulator && . ./venv/bin/activate && make PLATFORM=simulator -j {{jobs}}; \
    fi

    if [ -n "{{features}}"];then \
        cargo build --release --target={{current_target}} --lib;\
    else \
        cargo build --release --target={{current_target}} --lib --features {{features}};\
    fi

    just run_nwb

[confirm("This will clean the built app AND the simulator. Do you want to continue ?")]
clean-all:
    cd ./simulator && make clean
    cargo clean

[confirm("This will clean the built app AND DELETE the simulator. Do you want to continue ?")]
clear-all:
    rm -rf ./simulator
    cargo clean
