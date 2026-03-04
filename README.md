# NumworksAppsRust: One codebase, every firmwares

This project is a modern template to develop Rust apps on the Numworks calculator. It has built-in support for Epsilon NWA apps, Upsilon-external apps and the Epsilon simulator. The codebase for the 3 targets is exactly the same and all the conditional compilation is done in the NADK api.

## The most feature complete template for Numworks programming

NumworksAppsRust contains all the features from the Numworks' official EADK based Rust template and a lot of additional feature.

Here is a list of some available features:
- All the features that was in numworks-sample-app-rust
- Missing functions from the official sample app such as display_string
- A cleaner modules tree
- Documentation
- Up to date Nwlink
- Access to storage
- Higher level functions and structs
- Heap allocator
- Macros to make your code cleaner
- Pre-built InputManager, various rng functions, misc functions, and more!
- Fully functional simulator support with no additional code needed
- A true panic handler with debugging
- Automatic setup on Debian based Linux distros
- And more !

## Setup

First, you need to install the libs and build the dependencies. On Debian based distros, use:
```bash
sudo apt update
sudo apt install build-essential git imagemagick libx11-dev libxext-dev libfreetype6-dev libpng-dev libjpeg-dev pkg-config python3 python3-pip python3-venv curl libatomic1 gcc-arm-none-eabi binutils-arm-none-eabi -y
```

Next, clone the Epsilon repo with version 20:
```bash
git clone https://github.com/numworks/epsilon.git simulator -b version-20
```
Then, setup the Python venv:
```bash
python3 -m venv ./.venv
source ./.venv/bin/activate
pip3 install lz4 pypng stringcase
```

If Cargo is not installed, install it with:
```bash
# This will choose the default install. Feel free to customize your install.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
```

Then, you can install nodejs. You can install nodejs from the apt repositories (on Debian):
```bash
sudo apt install nodejs npm
```

But you can also install it using NVM:
```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
nvm install node
```

Next, setup the build targets for the calculator and install the Rust dependencies:
```bash
rustup target add thumbv7em-none-eabihf
cargo install just
```

If you are on a Debian based Linux Distro (Debian, Ubuntu, Linux Mint, ...), you can run `bash ./setup.sh` to install all the dependencies (You might have to reopen your terminal to reload the PATH). **For an unknown reason, this script can fail on the NVM install. If it happens to you, I suggest you to finish the install manually.**

And that's it! You should now be able to run `just sim` to see your creation comming to life in the simulator!
Available justfile commands:

| Command               | Description                                                                                                                              |
|-----------------------|------------------------------------------------------------------------------------------------------------------------------------------|
|`just build-epsilon`   | Build the app for **Epsilon** and create a NWA file that can be imported to the Numworks' [app page](https://my.numworks.com/apps).      |
|`just build-upsilon`   | Build the app for **Upsilon**.                                                                                                           |
|`just send-epsilon`    | Build and send the app to the calculator running **Epsilon**.                                                                            |
|`just send-upsilon`    | Build and send the app to the calculator running **Upsilon**.                                                                            |
|`just release-upsilon` | Build and make the project ready to be added to the [Upsilon-external](https://github.com/UpsilonNumworks/Upsilon-External/) repository. |
|`just check`           | Run `cargo check` for every targets and and every firmwares.                                                                             |
|`just sim [job-count]` | Build the app as a NWB and run the Epsilon simulator. The job count will be added to the cc `-j` argument (default 1).                   |
|`just clean`           | Clean the build cache for the app and the files used by Upsilon-External.                                                                |
|`just clear`           | Remove the build cache, the files for Upsilon-External and the simulator.                                                                |

## Quick start

The `src` folder contains a main.rs file and a nadk folder. The nadk folder is a rust module that contains the cross platform api.

The `main.rs` file uses 2 macros `setup_allocator!()` and `init_heap!();` in order to init the heap allocator.

You can find the list of all the available nadk's modules in `src/nadk/mod.rs`. The modules and the functions should be self explantory.

Note that the heap on Upsilon is only 80 Ko compared to the 100 Ko on Epsilon.

If you want to use a crate but only on calculator, for instance `alloc`, import this crate using the `calc_use!(crate)` macro. To import a crate only on the simulator, use `sim_use!(crate)`.

## Credits

NumworksAppsRust is inspired from the original numworks-sample-app-rust so it aims to fit to the original template but no code has been reused, so that's why NumworksAppsRust is under the MIT License.

I also used [storage.c by Yaya.cout](https://framagit.org/Yaya.Cout/numworks-extapp-storage) to access the storage of the calculator.

I reused a lot of features and code from [my NumcraftRust project](https://github.com/yannis300307/NumcraftRust) such has the simulator support or the allocator.

## Current state

NumworksAppsRust has been implemented in Numcraft and all the encountered bugs have been fixed. The template is not warranty to be 100 % bug free but is usable. If you encounter an issue, please open an issue on the Github repository.

## Need help?

You can ping me at `@yannis300307` on the Omega Community Discord server https://discord.gg/JpmjSH3.

## Licenses

The project is under the MIT License. The files in the build directory are under the MIT license but licensed by Damien Nicolet. (except build.rs) The files in src/nadk/storage are under the MIT license but licensed by Yaya.Cout. (except mod.rs)

Numworks is a registered trademark. This project has no association with Numworks.
