extern crate error_chain;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        FileNotFound(path: String) {
            description(path)
            display("File '{}' not found", path)
        }

        UnexpectedEOF(errmsg: String) {
            description(errmsg)
            display("Unexpected EOF: {}", errmsg)
        }
    }

    foreign_links {
        IoError(::std::io::Error);
    }
}