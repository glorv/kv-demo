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

        IllegalArgument(desc: String) {
            description(desc)
            display("Illegal argument: {}", desc)
        }

        IllegalState(desc: String) {
            description(desc)
            display("Illegal state: {}", desc)
        }
    }

    foreign_links {
        IoError(::std::io::Error);
        FromUtf8Err(::std::string::FromUtf8Error);
        GrpcioError(::grpcio::Error);
        ParseIntErr(::std::num::ParseIntError);
    }
}
