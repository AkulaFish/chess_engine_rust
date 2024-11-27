use std::fmt::{Debug, Display};

pub trait DisplayExtension {
    fn display(&self)
    where
        Self: Display,
    {
        println!("{}", self)
    }

    fn debug(&self)
    where
        Self: Debug,
    {
        println!("{:?}", self)
    }
}
