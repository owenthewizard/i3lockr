use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

/// Returns a closure that checks if the argument is greater than ```n```.
pub fn greater_than<T>(n: T) -> impl Fn(String) -> Result<(), String>
where
    T: Display + FromStr + PartialOrd,
    T::Err: ToString,
{
    move |s: String| {
        let num = match s.parse::<T>() {
            Ok(val) => val,
            Err(e) => return Err(e.to_string()),
        };
        if num > n {
            Ok(())
        } else {
            Err(format!("must be greater than {}", n))
        }
    }
}

/// Checks that ```s``` is the path to a valid PNG file.
/// Mostly unimplemented
// sadly can't be AsRef<OsStr> because of what structopt expects
pub fn is_png(s: &OsStr) -> Result<(), OsString> {
    eprintln!("Warning: PNG check bypassed");
    let metadata = match fs::metadata(s) {
        Ok(meta) => meta,
        Err(e) => return Err(e.to_string().into()),
    };
    if metadata.is_file() {
        Ok(())
    } else {
        Err("must be a PNG file".into())
    }
    //TODO add PNG header check or whatever
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greater_than_validator() {
        let four = "4".to_string();
        let five = "5".to_string();
        let six = "6".to_string();

        let greater_than_five = greater_than(5);

        assert!(greater_than_five(four).is_err());
        assert!(greater_than_five(five).is_err());
        assert!(greater_than_five(six).is_ok());
    }

    #[test]
    fn is_png_validator() {
        assert!(is_png(OsStr::new("/some/nonexist/file")).is_err());
        assert!(is_png(OsStr::new("/")).is_err());
        assert!(is_png(OsStr::new(".github/readme.png")).is_ok());
        unimplemented!();
    }
}
