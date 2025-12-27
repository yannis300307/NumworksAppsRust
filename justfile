app_name := "SampleApp"
lib_file_name := "libsample_app"

icon_file := "assets/icon.png"

current_target := shell("rustc -vV | grep \"host:\" | awk '{print $2}'")

build-epsilon:
    cargo build --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "epsilon" --no-default-features

build-upsilon:
    mkdir -p target/upsilon_api
    make -f build/upsilon-api/Makefile
    cargo build --release --bin {{app_name}} --target=thumbv7em-none-eabihf --features "upsilon" --no-default-features

send-epsilon: build-epsilon
    npm exec --yes -- nwlink@0.0.19 install-nwa ./target/thumbv7em-none-eabihf/release/{{app_name}}

send-upsilon:
    just build-upsilon
    # Code adapted from https://github.com/UpsilonNumworks/Upsilon-External/blob/master/Makefile. Under MIT
    ./build/archive apps.tar {{app_name}}
    echo "Waiting for the calculator to be connected, use the bootloader to flash on Upsilon if your app is bigger than 2MB"
    until dfu-util -l | grep -E "0483:a291|0483:df11" > /dev/null 2>&1; do sleep .5;done
    dfu-util -i 0 -a 0 -s 0x90200000 -D target/apps.tar

release-upsilon:
    just build-upsilon
    . ./.venv/bin/activate && python3 ./build/png2icon.py {{icon_file}} app.icon
    cp ./target/thumbv7em-none-eabihf/release/{{app_name}} ./app.elf
    cp ./assets/icon.png ./icon.png

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
        cd simulator && . ../.venv/bin/activate && make PLATFORM=simulator -j {{jobs}}; \
    fi
    cargo build --release --target={{current_target}} --lib --features "epsilon" --no-default-features
    just run_nwb

[confirm("This will clean the built app. Do you want to continue ?")]
clean:
    rm -f ./app.elf ./app.icon
    cargo clean

[confirm("This will clean the built app AND DELETE the simulator. Do you want to continue ?")]
clear:
    rm -rf ./simulator
    rm -f ./app.elf ./app.icon ./icon.png
    cargo clean
