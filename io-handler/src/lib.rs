#![no_std]

pub async fn pin_output_task<S, P>(signal: &mut S, pin: &mut P) -> !
where
    S: channel::Rx<Item = bool>,
    P: embedded_hal::digital::OutputPin,
{
    loop {
        if let Ok(s) = signal.recv().await {
            pin.set_state(s.into());
        }
    }
}

pub async fn pin_input_task_polling<S, P, D>(signal: &mut S, pin: &mut P, delay: &mut D) -> !
where
    S: channel::Tx<Item = bool>,
    P: embedded_hal::digital::InputPin,
    D: embedded_hal_async::delay::DelayNs,
{
    loop {
        delay.delay_ms(10).await;
        if let Ok(s) = pin.get_state() {
            signal.send(s.into()).await;
        }
    }
}
