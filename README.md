# swf-patcher

CLI for bulk exporting (and soon patching) SWF content using ffdec-cli recursively, automating the process and maintaining the directory structure.

## Installation

Currently only Windows is supported

Make sure you have ffdec installed and added to your PATH, you should be able to run `ffdec-cli` in your terminal

You can install `swf-patcher` using Cargo:

```sh
cargo install --git https://github.com/bluisblu/swf-patcher
```

Otherwise, you can clone the repository and run it using Cargo


## Usage

Ensure you have your .swf files in the `original` directory, for example if you're exporting from U.B. Funkeys:

```
.
└── original
    ├── Main.swf
    ├── MainAS3.swf
    └── games
        ├── 3dMatchGame.swf
        ├── a1p_boxing.swf
        ├── a1p_ccheckers.swf
        └── ...
```

Note: If you are exporting from U.B. Funkeys, it will take a long time to export all the content (it took roughly an hour for me to export all scripts). I recommend only exporting the content you need from the `.swf` files you need it from. Certain files, like the funkeys themselves, are mainly there for the animation frames.


You can then get a list of available export types by running:

```sh
# If installed globally
swf-patcher list-types

# If running from repository
cargo run -- list-types
```

And then export them using:

```sh
# If installed globally
swf-patcher export <type>

# If running from repository
cargo run -- export <type>
```
