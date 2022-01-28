pub struct FooStruct<'a> {
    pub bars: Vec<&'a str>,
}

pub trait FooTrait<'a> {
    fn getBars(&self) -> &'a Vec<&'a str>;
}

impl<'a> FooTrait<'a> for FooStruct<'a> {
    fn getBars(&self) -> &'a Vec<&'a str> {
        &self.bars // cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
    }
}

fn main() {}
