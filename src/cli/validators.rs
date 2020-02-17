pub fn has_compose(_: String) -> Result<(), String> {
    if cfg!(feature = "png") || cfg!(feature = "jpeg") {
        Ok(())
    } else {
        Err("Must have 'png' or 'jpeg' features enabled to compose images".to_owned())
    }
}
