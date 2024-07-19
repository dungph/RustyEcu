#![no_std]

pub async fn can_rx_task<S, C>(subs: &mut S, can: &mut C) -> !
where
    S: channel::Tx<Item = ()>,
    C: channel::Rx<Item = ()>,
{
    loop {
        can.recv().await.ok();
        subs.send(()).await.ok();
    }
}

pub async fn can_tx_task<S, C>(subs: &mut S, can: &mut C) -> !
where
    S: channel::Rx<Item = ()>,
    C: channel::Tx<Item = ()>,
{
    loop {
        subs.recv().await.ok();
        can.send(()).await.ok();
    }
}
