app_name := "SampleApp"
lib_file_name := "libsample_app"

current_target := shell("rustc -vV | grep \"host:\" | awk '{print $2}'")

build-epsilon:
    cargo build --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "epsilon" --no-default-features

build-upsilon:
    cargo build --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "upsilon" --no-default-features

send-epsilon:
    cargo build --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "epsilon" --no-default-features
    npm exec --yes -- nwlink@0.0.19 install-nwa ./target/thumbv7em-none-eabihf/release/{{app_name}}

check:
    cargo check --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "epsilon" --no-default-features
    cargo check --release --target={{current_target}} --lib --features "epsilon" --no-default-features
    cargo check --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "upsilon" --no-default-features
    cargo check --release --target={{current_target}} --lib --features "upsilon" --no-default-features
    @echo All checks passed!

[macos]
run_nwb:
    ./simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/release/{{lib_file_name}}.dylib

[linux]
run_nwb:
    ./simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/release/{{lib_file_name}}.so

sim jobs="1":
    if [ ! -f "./simulator/output/release/simulator/linux/epsilon.bin" ]; then \
        cd simulator && . ./.venv/bin/activate && make PLATFORM=simulator -j {{jobs}}; \
    fi
    cargo build --release --target={{current_target}} --lib --features "epsilon" --no-default-features
    just run_nwb

[confirm("This will clean the built app AND the simulator. Do you want to continue ?")]
clean-all:
    cd ./simulator && make clean
    cargo clean

[confirm("This will clean the built app AND DELETE the simulator. Do you want to continue ?")]
clear-all:
    rm -rf ./simulator
    cargo clean
