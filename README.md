# Banana Prince Password Generator (Source)

This is the full source code for the password generator I made. [Try it out here](https://fcard.github.io/BananaPassword/).

Banana Prince is a NES game, it's pretty cool! I figured out the algorithm it uses on its password system so I decided to make a little thing out of it, altho I ended up getting carried away and put way too much effort into it, hehe. I hope you have fun using it!

# Building (& Running Locally)

This is a Rust/Yew app, to build it you need Rust & Cargo & Trunk. You can install the first two via the instructions on [the official Rust website](www.rust-lang.org). Once you have cargo, install Trunk on the commmand line via:
```
cargo install trunk
```

To then download and build the app, run:
```
git clone https://github.com/fcard/BananaPasswordSrc
cd BananaPasswordSrc
trunk build --release --public-url BananaPassword
```

This should create a dist folder in the source's root directory, which is ready for upload on github pages.

To run the page locally, go to the source's root direcgory and run:
```
trunk serve --open
```

That should open the page on your default web-browser. Just running
```
trunk serve
```
Will start let you access the page via the address `127.0.0.1:8080`, so you can pick how you want to view it.

# Command Line Version

This app doubles as a desktop command line program. To build it, in the main root directory run:
```
cargo build --release
```

This will build a binary `target/release/banana_pass`. You can also install it directly with:
```
cargo install --path .
```

This is the usage:
```
banana_pass [-i] [-s <world>-<level>] [-t <treasure>] [-w <weapon>] [-p <print_style>]
  -s, --stage:        sets the stage, between 1-1 to 7-3

  -t, --treasure:     sets the amount of treasures, can be a number or name
                        possible values: 0, 1, 2, 3, 4,
                                         none, sword, shield, armor, crown

  -w, --weapon:       sets the current weapon, can be a number or name
                      dashes in names can be substitued by a space or nothing
                        possible values: 0 through 15,
                                         stone-axe, king-axe, star-axe,
                                         magic, capsule, maxim, meteor,
                                         megalo, fold, cross-axe,
                                         grand-axe, curse-axe, mercury,
                                         flash, crusher, kingship

  -p, --print:        sets the print style of the password.
                        possible values:
                          full: default value, prints a verbose password
                          compact: same as full, but with much less spacing
                          numbers: print the password as numbers
                          none: does a dry run, i.e. doesn't print anything

  -i, --interactive:  instead of printing the password once, start an
                      interactive mode. other options will set the
                      initial password.
```

Altenatively, you can run `banana_pass -i` to enter an interactive mode.

