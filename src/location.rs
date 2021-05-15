pub trait Location {
    fn set_start(&mut self, row: usize, column: usize);
    fn set_end(&mut self, row: usize, column: usize);
}
