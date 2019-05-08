use std::fmt::Display;
use std::process;

pub trait Anticipate {
    type Item;

    /// Anticipate an value and return it if present. Otherwise, non-zero exit and spit out `msg`
    /// along with the unanticipated error.
    fn anticipate(self, msg: &str) -> Self::Item;
}

pub trait AnticipateErr {
    type ErrItem;

    /// Anticipate an error and return it if present. Otherwise, non-zero exit and spit out `msg`
    /// along with the unanticipated error.
    fn anticipate_err(self, msg: &str) -> Self::ErrItem;
}

impl<A, E> Anticipate for Result<A, E>
where
    E: Display,
{
    type Item = A;

    fn anticipate(self, msg: &str) -> Self::Item {
        match self {
            Ok(val) => val,
            Err(err) => {
                eprintln!("{}: {}", msg, err);
                process::exit(1);
            }
        }
    }
}

impl<A, E> AnticipateErr for Result<A, E>
where
    A: Display,
{
    type ErrItem = E;

    fn anticipate_err(self, msg: &str) -> Self::ErrItem {
        match self {
            Err(err) => err,
            Ok(val) => {
                eprintln!("{}: {}", msg, val);
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
