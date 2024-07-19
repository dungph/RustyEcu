#![no_std]

use core::pin::pin;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embedded_hal_async::delay::DelayNs;
use futures::future::{select, Either};

static LEFT_SIG: Channel<CriticalSectionRawMutex, bool, 1> = Channel::new();
static RIGHT_SIG: Channel<CriticalSectionRawMutex, bool, 1> = Channel::new();

pub async fn blinker_left_control_task<E>(sig: &mut E) -> !
where
    E: channel::Rx<Item = bool>,
{
    loop {
        if let Ok(state) = sig.recv().await {
            LEFT_SIG.send(state).await;
        }
    }
}

pub async fn blinker_right_control_task<E>(sig: &mut E) -> !
where
    E: channel::Rx<Item = bool>,
{
    loop {
        if let Ok(state) = sig.recv().await {
            LEFT_SIG.send(state).await;
        }
    }
}

pub async fn blinker_control_task<L, R>(left_sig: &mut L, right_sig: &mut R) -> !
where
    L: channel::Rx<Item = bool>,
    R: channel::Rx<Item = bool>,
{
    loop {
        let left_fut = pin!(left_sig.recv());
        let right_fut = pin!(right_sig.recv());
        let res = select(left_fut, right_fut).await;
        match res {
            Either::Left((Ok(state), _)) => LEFT_SIG.send(state).await,
            Either::Right((Ok(state), _)) => RIGHT_SIG.send(state).await,
            _ => (),
        }
    }
}

pub async fn blinker_left_task<L>(led: &mut L) -> !
where
    L: channel::Tx<Item = bool>,
{
    loop {
        let state = LEFT_SIG.receive().await;
        led.send(state).await;
    }
}
pub async fn blinker_right_task<L>(led: &mut L) -> !
where
    L: channel::Tx<Item = bool>,
{
    loop {
        let state = RIGHT_SIG.receive().await;
        led.send(state).await;
    }
}
