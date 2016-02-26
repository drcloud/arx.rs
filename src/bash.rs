use std::ffi::CString;


pub enum Fragment {
    ArgV(Vec<CString>),
    And(Box<Fragment>, Box<Fragment>),
    Or(Box<Fragment>, Box<Fragment>),
    Wraps(Box<Fragment>, Box<Fragment>),
}
