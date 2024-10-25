use colourful_logger::Logger;
use serde::Serialize;

#[derive(Serialize)]
struct Struct1 {
    field1: String,
    field2: i32,
}

#[derive(Serialize)]
struct Struct2 {
    field1: String,
    field2: i32,
}

#[derive(Serialize)]
struct Struct3 {
    field1: String,
    field2: i32,
}

#[derive(Serialize)]
struct Struct4 {
    field1: String,
    field2: i32,
}

#[derive(Serialize)]
struct Struct5 {
    field1: String,
    field2: i32,
}

fn main() {
    let struct1 = Struct1{ field1: "some random value 1".to_string(), field2: 540 };
    let struct2 = Struct2{ field1: "some random value 2".to_string(), field2: 69 };
    let struct3 = Struct3{ field1: "some random value 3".to_string(), field2: 5 };
    let struct4 = Struct4{ field1: "some random value 4".to_string(), field2: 3495843 };
    let struct5 = Struct5{ field1: "some random value 5".to_string(), field2: 594834594 };
    let log = Logger::new(colourful_logger::LogLevel::Silly, Some("joe"));

    log.info("Info Message!", "Main", true, struct1);
    log.debug("Debug Message!", "Main", false, struct2);
    log.error("Error Message!", "Main",false, struct3);
    log.fatal("Fatal Message!", "Main",false, struct4);
    log.silly("Silly Message!", "Main",false, struct5);
    log.warn("Warn Message!", "Main", false, "Joe".to_string());

    log.info_single("Single Info Message!", "Main");
    log.debug_single("Single Debug Message!", "Main");
    log.error_single("Single Error Message!", "Main");
    log.fatal_single("Single Fatal Message!", "Main");
    log.silly_single("Single Silly Message!", "Main");
    log.warn_single("Single Warn Message!", "Main");

}