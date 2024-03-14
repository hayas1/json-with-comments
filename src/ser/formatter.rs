pub trait JsoncFormatter {}

pub struct MinifyFormatter {}
impl JsoncFormatter for MinifyFormatter {}

pub struct PrettyFormatter {}
impl JsoncFormatter for PrettyFormatter {}
