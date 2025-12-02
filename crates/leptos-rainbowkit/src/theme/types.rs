pub trait Theme {
    fn name(&self) -> &'static str;
    fn css_vars(&self) -> &'static str;
}
