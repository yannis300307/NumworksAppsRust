# Meet : A better template for the futur of Rust programming on Numworks calculator

Here is our new innovation: A fully featured Numworks third party app template made for awesome Rust programmers, just like you!

With this new product, the only limit is your imagination!

With the incredible name of NumworksAppsRust, our product has a lot more features than the old numworks-sample-app-rust with up to date dependencies.

# Why NumworksAppsRust is one of the most feature complete template for Numworks programming?

Our new product features:
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

But what about the ease of use?

# How NumworksAppsRust reinvented the art of programming using Rust on calculator?

We designed NumworksAppsRust to be has easy to use as possible.

If you are on a Debian based Linux Distro (Debian, Ubuntu, Linux Mint, ...), you simply have to run `bash ./setup.sh` to install all the dependencies (You might have to reopen your terminal to reload the PATH).

And that's it! You should now be able to run `just sim` to see your creation comming to life in the simulator!

To build your app, run `just build`.
To send it to your calculator use `just send`.
To check your rust code for all the targets, run `just check`.
If you are borred to have a `target` folder of around 42 TB, you can run `just clean-all`.
If you want to clear all the build cache (including the simulator), run `just clear-all`.

# How we made NumworksAppsRust?

NumworksAppsRust is inspired from the original numworks-sample-app-rust so it aims to fit to the original template but no code has been reused, so that's why NumworksAppsRust is under the MIT License.

We also used [storage.c by Yaya.cout](https://framagit.org/Yaya.Cout/numworks-extapp-storage) to access the storage of the calculator.

We reused a lot of features and code from [Yannis's NumcraftRust project](https://github.com/yannis300307/NumcraftRust) such has the simulator support or the allocator.

# How to contact us?

You can ping us (me, in fact) at `@yannis300307` on the Omega Community Discord serve https://discord.gg/JpmjSH3.
