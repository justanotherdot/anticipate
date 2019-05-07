use std::fmt::Display;
use std::process;

pub trait Anticipate {
    type Item;

    /// If a value is present in `self`, return it.
    /// Otherwise, exit(1) and spit out `msg`.
    fn anticipate(self, msg: &str) -> Self::Item;

    /// If a value is present in `self`, return it.
    /// Otherwise, exit(1) and spit out `msg` along
    /// with an error if it's present.
    fn anticipate_err(self, msg: &str) -> Self::Item;
}

impl<A, E: Display> Anticipate for Result<A, E> {
    type Item = A;

    fn anticipate(self, msg: &str) -> Self::Item {
        match self {
            Ok(val) => val,
            Err(_err) => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }

    fn anticipate_err(self, msg: &str) -> Self::Item {
        match self {
            Ok(val) => val,
            Err(err) => {
                eprintln!("{}: {}", msg, err);
                process::exit(1);
            }
        }
    }
}

impl<A> Anticipate for Option<A> {
    type Item = A;

    fn anticipate(self, msg: &str) -> Self::Item {
        match self {
            Some(val) => val,
            None => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }

    fn anticipate_err(self, msg: &str) -> Self::Item {
        self.anticipate(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Since we are spitting out to stderr
    // it's possibly better to wrap this in a dummy binary
    // and golden test it's output
    #[test]
    fn anticipate_result() {
        let val = 12;
        let result: Result<isize, &str> = Ok(val);
        assert_eq!(result.anticipate("not present"), val);
    }

    #[test]
    fn anticipate_err_result() {
        let val = 12;
        let result: Result<isize, &str> = Ok(val);
        assert_eq!(result.anticipate_err("not present"), val);
    }

    #[test]
    fn anticipate_option() {
        let val = 12;
        let result: Option<isize> = Some(val);
        assert_eq!(result.anticipate("not present"), val);
    }

    #[test]
    fn anticipate_err_option() {
        let val = 12;
        let result: Option<isize> = Some(val);
        assert_eq!(result.anticipate_err("not present"), val);
    }
}
