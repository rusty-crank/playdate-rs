use core::cell::UnsafeCell;

use rand::{rngs::SmallRng, SeedableRng};
use spin::Lazy;

use crate::PLAYDATE;

/// Get global random number generator.
pub fn rng() -> impl rand::Rng {
    unsafe { &mut *RNG.0.get() }
}

fn generate_seed() -> u64 {
    static mut COUNTER: u64 = 0;
    let mut seed = unsafe {
        COUNTER += 1;
        COUNTER
    };
    seed += PLAYDATE.system.get_timezone_offset().unsigned_abs() as u64;
    // let x = self.get_accelerometer();
    // seed += x.0 as u64;
    // seed += x.1 as u64;
    // seed += x.2 as u64;
    seed += PLAYDATE.system.get_current_time_milliseconds() as u64;
    seed += PLAYDATE.system.get_elapsed_time() as u64;
    seed += PLAYDATE.system.get_battery_percentage() as u64;
    seed += PLAYDATE.system.get_crank_angle() as u64;
    seed += PLAYDATE.system.get_button_state().current.bits() as u64;
    seed
}

struct SmallRngCell(UnsafeCell<SmallRng>);

unsafe impl Send for SmallRngCell {}
unsafe impl Sync for SmallRngCell {}

static RNG: Lazy<SmallRngCell> =
    Lazy::new(|| SmallRngCell(UnsafeCell::new(SmallRng::seed_from_u64(generate_seed()))));
