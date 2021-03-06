
// Module for errors using error_chain
// Might be expanded in the future

error_chain! {
    errors {
        ConfigParseError(filename: String) {
            description("configuration file could not be parsed"),
            display("configuration file could not be parsed: '{}'", filename),
        }
    }
}

