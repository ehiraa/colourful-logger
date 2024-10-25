# Colourful-Logger 2.0

The Colourful-Logger is a simple yet effective logging utility designed to enhance the readability of log messages by incorporating vibrant colors.
Allowing you to also print structs, strings and other important types, either to the terminal or to a file.

## Features
- [x] Easy to use
- [x] Colour coded log levels
- [x] Quick identification of log types
- [x] Enhanced readability
- [x] Simple integration into projects
- [x] Immediate visual improvement
- [x] Terminal or File Logging
- [x] Log any seralised data structures
- [x] Log filtering

## How to use
You can use either lazy_static! to use the logger as a global variable
Colourful logger also has a built in default(), if you want a preset Logging.

> [!IMPORTANT]
> If you want to log structs, which are typically not supported automatically
> Add `serde = { version = "1.0.213", features = ["derive"] }` to your `Cargo.toml`
> And then append `#[derive(Serialize)]` above your struct to seralise it. <br/>
> It is then accessible to the logger as an object.

You may change the log file, remove the log file or even change the LogLevel at any time.

### Change File and LogLevel
```rust
use colourful_logger::{Logger, LogLevel};

fn main() {
    let mut logger = Logger::default();

    logger.set_file("file_name.log");
    logger.remove_file();
    logger.set_log_level(LogLevel::Warn);
}
```

### Without Default, but Lazy Static!

```rust
use colourful_logger::{Logger, LogLevel};
use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
    // Keep log_file as "" so that it doesn't log to the file.
    // If you want it to then be sure to type in the file name and extension.
    static ref LOGGER: Logger = Logger::new(LogLevel::Info, Some(""));
}

#[derive(Serialize)]
struct RandomStruct {
    field1: String,
    field2: i32,
}

fn main(): {
    let random_struct = RandomStruct{ field1: "some random value 1".to_string(), field2: 540 };
    LOGGER.info("This is a message!", "Tag", false, random_struct);
    LOGGER.info("Another message!", "Main", true, "Joe".to_string());
    LOGGER.info_single("This is a single message!", "Hello");

    // Output
    // [2024-10-25 07:08:11] info: ┏ [Tag] This is a message!
    //                              ┗ [1] {"field1":"some random value 2","field2":69}
    //
    // [2024-10-25 07:08:11] info:  ┏ [Main] Another message!
    //                              ┃ at main.rs:42:5 [colourful_logger::main::h64e1f92e8d679d92]
    //                              ┗ [1] "Joe"
    //
    // [2024-10-25 07:08:11] info:  ▪ [Hello] This is a single message!
}
```

### With Default, without Lazy Static!

```rust
use colourful_logger::Logger as Logger;

#[derive(Serialize)]
struct RandomStruct {
    field1: String,
    field2: i32,
}

fn main(): {
    let logger = Logger::default();

    logger.info("This is a message!", "Tag", false, random_struct);
    logger.info("Another message!", "Main", true, "Joe".to_string());
    logger.info_single("This is a single message!", "Hello");

    // Output
    // [2024-10-25 07:08:11] info: ┏ [Tag] This is a message!
    //                              ┗ [1] {"field1":"some random value 2","field2":69}
    //
    // [2024-10-25 07:08:11] info:  ┏ [Main] Another message!
    //                              ┃ at main.rs:42:5 [colourful_logger::main::h64e1f92e8d679d92]
    //                              ┗ [1] "Joe"
    //
    // [2024-10-25 07:08:11] info:  ▪ [Hello] This is a single message!
}
```

## Bug Reports | Features
If there are any bugs, or features you'd like to implement into the logger, feel free to create a pr request and it'll be looked into.

## License
This project uses the following license: [MIT LICENSE](https://github.com/devtomos/colourful-logger/blob/main/README.md).