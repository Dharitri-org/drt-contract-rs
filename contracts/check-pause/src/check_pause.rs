#![no_std]

dharitri_sc::imports!();

use dharitri_sc_modules::pause;

#[dharitri_sc::contract]
pub trait CheckPauseContract: pause::PauseModule {
    #[init]
    fn init(&self) {}

    #[endpoint(checkPause)]
    fn check_pause(&self) -> SCResult<bool> {
        Ok(self.is_paused())
    }
}
