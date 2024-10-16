// use core::f64::consts::PI;
// use defmt::println;
// use embassy_rp::gpio::{AnyPin, Output, Level};
// use embassy_rp::Peripherals;
// use embassy_time::Timer;
// use libm::{acos, pow, round, fabs};
// use crate::coordinate::PolarCoordinate;
// use crate::stepper::Stepper;
//
//
// const MAIN_PULLEY_TEETH: i8 = 90;
// const MOTOR_PULLEY_TEETH: i8 = 16;
// const DEGREES_PER_STEP: f64 = 1.8;
// const STEPS_PER_DEG: f64 = MAIN_PULLEY_TEETH as f64
//     / MOTOR_PULLEY_TEETH as f64
//     / DEGREES_PER_STEP;
//
// struct StepPosition {
//     primary_steps: i64,
//     secondary_steps: i64,
// }
//
// impl StepPosition {
//     pub fn get_total_steps(&self) -> i64 {
//         let primary_total_steps = abs(self.primary_steps);
//         let secondary_total_steps = abs(self.secondary_steps);
//         primary_total_steps + secondary_total_steps
//     }
//
//     pub fn delta(&self, other: &StepPosition) -> StepPosition {
//         StepPosition {
//             primary_steps: self.primary_steps - other.primary_steps,
//             secondary_steps: self.secondary_steps - other.secondary_steps,
//         }
//     }
// }
//
// pub struct Arm<'a> {
//     step_position: StepPosition,
//     primary_stepper: Stepper<'a>,
//     secondary_stepper: Stepper<'a>,
//     enable_output: Output<'a, AnyPin>,
// }
//
// impl Arm<'_> {
//     pub fn new(p: Peripherals) -> Self {
//         let s0_step = AnyPin::from(p.PIN_12);
//         let s0_dir = AnyPin::from(p.PIN_13);
//         let s1_step = AnyPin::from(p.PIN_14);
//         let s1_dir = AnyPin::from(p.PIN_15);
//         let enable_pin = AnyPin::from(p.PIN_18);
//         let mut enable_output = Output::new(enable_pin, Level::Low);
//
//         Arm {
//             step_position: get_target_step_position(&PolarCoordinate {
//                 theta: 0.0,
//                 rho: 0.0,
//             }),
//             primary_stepper: Stepper::new(s0_step, s0_dir),
//             secondary_stepper: Stepper::new(s1_step, s1_dir),
//             enable_output,
//         }
//     }
//
//     pub async fn move_to(&mut self, position: &PolarCoordinate) {
//         self.enable_output.set_low();
//         let mut delay = 20000000.0;
//         let target_step_position = get_target_step_position(position);
//
//         loop {
//             let delta_step_position = target_step_position.delta(&self.step_position);
//
//             if delta_step_position.get_total_steps() == 0 {
//                 // already at target position
//                 return;
//             }
//
//             let primary_step_delta = delta_step_position.primary_steps;
//             let primary_step_delta_abs = abs(primary_step_delta);
//             let secondary_step_delta = delta_step_position.secondary_steps;
//             let secondary_step_delta_abs = abs(secondary_step_delta);
//
//             let mut partial_step = 0.0;
//             let is_primary_faster = primary_step_delta_abs > secondary_step_delta_abs;
//             let primary_direction = match primary_step_delta_abs {
//                 0 => 0,
//                 _ => primary_step_delta / primary_step_delta_abs
//             };
//             let secondary_direction = match secondary_step_delta_abs {
//                 0 => 0,
//                 _ => secondary_step_delta / secondary_step_delta_abs
//             };
//
//             // FIXME: unnecessary duplication/branching, lost a battle with the borrow checker :(
//             if is_primary_faster {
//                 let speed_ratio = secondary_step_delta_abs as f64 / primary_step_delta_abs as f64;
//                 self.primary_stepper.step(primary_direction);
//                 self.step_position.primary_steps += primary_direction;
//
//                 partial_step += speed_ratio;
//
//                 while partial_step >= 0.0 {
//                     self.secondary_stepper.step(secondary_direction);
//                     self.step_position.secondary_steps += secondary_direction;
//                     partial_step -= 1.0;
//                 }
//             } else {
//                 let speed_ratio = primary_step_delta_abs as f64 / secondary_step_delta_abs as f64;
//
//                 self.secondary_stepper.step(secondary_direction);
//                 self.step_position.secondary_steps += secondary_direction;
//
//                 partial_step += speed_ratio;
//
//                 while partial_step >= 0.0 {
//                     self.primary_stepper.step(primary_direction);
//                     self.step_position.primary_steps += primary_direction;
//                     partial_step -= 1.0;
//                 }
//             }
//
//             Timer::after_nanos(fabs(delay) as u64).await;
//             delay *= 0.9;
//         }
//     }
// }
//
// fn get_target_step_position(position: &PolarCoordinate) -> StepPosition {
//     let PolarCoordinate { theta, rho } = position;
//     let theta_degrees = degrees(*theta);
//     let secondary_degrees = 180.0 - degrees(acos((0.5 - pow(*rho, 2.0)) * 2.0));
//     let primary_offset = secondary_degrees / 2.0;
//     let primary_degrees = theta_degrees - primary_offset;
//
//     let primary_steps = round(primary_degrees * STEPS_PER_DEG) as i64;
//     StepPosition {
//         primary_steps,
//         secondary_steps: round(secondary_degrees * STEPS_PER_DEG) as i64 + primary_steps,
//     }
// }
//
// fn degrees(radians: f64) -> f64 {
//     radians * (180.0 / PI)
// }
//
// fn abs(val: i64) -> i64 {
//     fabs(val as f64) as i64
// }
