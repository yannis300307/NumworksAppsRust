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

If you are on a Debian based Linux Distro (Debian, Ubuntu, Linux Mint, ...), you simply have to run `bash ./setup.sh` to install all the dependencies (You might have to reopen your terminal to reload the PATH).

And that's it! You should now be able to run `just sim` to see your creation comming to life in the simulator!

To build your app, run `just build`.
To send it to your calculator use `just send-epsilon` or `just send-upsilon` depending on your calcualator's firmware.
To check your rust code for all the targets, run `just check`.
If you are borred to have a `target` folder of around 42 TB, you can run `just clean-all`.
If you want to clear all the build cache (including the simulator), run `just clear-all`.

## Quick documentation

The `src` folder contains a main.rs file and a nadk folder. The nadk folder is rust module that contain the cross platform api.

The `main.rs` file uses 2 macros `setup_allocator!()` and `init_heap!();` in order to init the heap allocator.

You can find the list of all the available nadk's modules in `src/nadk/mod.rs`. The modules and the functions should be self explantory.

Note that the heap on Upsilon is only 80 Ko compared to the 100 Ko on Epsilon.

If you want to use a crate but only on calculator, for instance `alloc`, import this crate using the `calc_use!(crate)` macro. To import a crate only on the simulator, use `sim_use!(crate)`.

## Credits

NumworksAppsRust is inspired from the original numworks-sample-app-rust so it aims to fit to the original template but no code has been reused, so that's why NumworksAppsRust is under the MIT License.

We also used [storage.c by Yaya.cout](https://framagit.org/Yaya.Cout/numworks-extapp-storage) to access the storage of the calculator.

We reused a lot of features and code from [Yannis's NumcraftRust project](https://github.com/yannis300307/NumcraftRust) such has the simulator support or the allocator.

## Current state

The icons are not supported on Upsilon but I will add that soon.

NumworksAppsRust has been implemented in Numcraft and all the encountered bugs have been fixed. The template is not warranty to be 100 % bug free but is usable. If you encounter an issue, please open an issue on the Github repository.

## Need help?

You can ping me at `@yannis300307` on the Omega Community Discord serve https://discord.gg/JpmjSH3.

## Licenses

The project is under the MIT License. The files in the build directory are under the MIT license but licensed by Damien Nicolet. (except build.rs) The files in src/nadk/storage are under the MIT license but licensed by Yaya.Cout. (except mod.rs)

Numworks is a registered trademark. This project has no association with Numworks.