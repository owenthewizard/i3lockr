use std::fmt::Display;
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

pub fn has_compose(_: String) -> Result<(), String> {
    if cfg!(feature = "png") || cfg!(feature = "jpeg") {
        Ok(())
    } else {
        Err("Must have 'png' or 'jpeg' features enabled to compose images".to_owned())
    }
}

/*
/// Returns a closure that checks if the argument is between two numbers.
pub fn between<T>(small: T, large: T) -> impl Fn(String) -> Result<(), String>
where
    T: Display + FromStr + PartialOrd,
    T::Err: ToString,
{
    move |s: String| {
        let num = match s.parse::<T>() {
            Ok(val) => val,
            Err(e) => return Err(e.to_string()),
        };
        if num >= large {
            return Err(format!("must be smaller than {}", large));
        }
        if num <= small {
            return Err(format!("must be greater than {}", small));
        }
        Ok(())
    }
}
*/

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

    /*
    #[test]
    fn between_validator() {
        let forty_one = "41".to_string();
        let forty_two = "42".to_string();
        let forty_three = "43".to_string();

        let at_42 = between(41, 43);

        assert!(at_42(forty_one).is_err());
        assert!(at_42(forty_two).is_ok());
        assert!(at_42(forty_three).is_err());
    }
    */
}
