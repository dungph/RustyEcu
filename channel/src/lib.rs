#![no_std]

pub trait Channel {
    type Item;
    type Err;
}

pub trait Tx: Channel {
    async fn send(&mut self, item: Self::Item) -> Result<(), Self::Err>;
}

pub trait Rx: Channel {
    async fn recv(&mut self) -> Result<Self::Item, Self::Err>;
}
