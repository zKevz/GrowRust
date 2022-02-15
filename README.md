# GrowRust
Growtopia Private Server made in Rust.
# Grow Rust
**Grow Rust is a Growtopia Private Server made in Rust.**

## Building
```console
$ git clone https://github.com/zKevz/GrowRust
$ cd GrowRust
$ cargo build
```

Or `cargo build --release` for release mode and code optimization

After you've built it, create a directory named `data` in the same directory as your executable in. After that, put `items.dat` which can be found in the Growtopia local folder/cache in that `data` directory. Then simply just run the executable and it should work!

## Features
- Player Database
- World Database
- Account creation, login
- World entering
- Breaking and placing blocks
- Respawn
- Chat and commands (only /test and /give)
- Clothes
- Multiplayer of course
- Clean code
- Fast and safe of course!

## Why Rust?
Because im interested in rust these days, but been struggling in what project what i should do. And then the idea came from the very first Growtopia Private Server in rust made by Alexander ( [Growtopia.rs](https://github.com/Alexander9673/Growtopia.rs) ). So im thinking of making a brand new server in rust with even more features than Alexander's has, which is why there is this project!

## Small Documentation
Calling a variant function:
```rust
use utils::variant_function::VariantFunction::*; // in the top of the file!

player.send_varfn(OnConsoleMessage("Hello!"));
```

However, you can explicitly calls the `VariantFunction` enum by
```rust
use utils::variant_function::VariantFunction; // in the top of the file!

player.send_varfn(VariantFunction::OnConsoleMessage("Hello!"));
```

And then for variant calls with `netid` and `delay` arguments:
```rust
use utils::variant_function::VariantFunction::*; // in the top of the file!

let net_id = -1;
let delay = 6969;
player.send_varfn_v(OnConsoleMessage("Hello!"), net_id, delay);
```

To create another implementation variant function, you can add it in `VariantFunction` enum and then put the field with the arguments provided, and then create the serialization implementation in `serialize` function. You can see many examples there, it should be easy enough.

## Contributing
Feel free to contribute in the development of the server! If you have found any bugs, feel free to fix or contact me. Any help would be appreciated.

## Credits
[Abood](https://github.com/AboodTBR) (Definitely a cool guy)

[Mempler](https://github.com/Mempler)

[Alexander#9673](https://github.com/Alexander9673)

[free#1234](https://github.com/smhman)

[enet-sys](https://github.com/ruabmbua/enet-sys)

[enet-rs](https://github.com/futile/enet-rs) (Some implementation were taken from here)
