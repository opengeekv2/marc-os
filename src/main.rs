#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]


use core::panic::PanicInfo;

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::uart::UARTDriver::new(crate::uart::internal::UART::new(0x1000_0000)), $($args)+);
	});
}
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}	

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}

#[cfg(not(test))]
#[no_mangle]
extern "C" fn kmain() {

	let my_uart = uart::UARTDriver::new(uart::internal::UART::new(0x1000_0000));

	println!("This is my operating system!");
	println!("I'm so awesome. If you start typing something, I'll show you what you typed!");
	
	loop {
		if let Some(c) = my_uart.get() {
			match c {
				8 => {
					// This is a backspace, so we essentially have
					// to write a space and backup again:
					print!("{}{}{}", 8 as char, ' ', 8 as char);
				},
				10 | 13 => {
					// Newline or carriage-return
					println!();
				},
				_ => {
					print!("{}", c as char);
				}
			}
		}
	}	
}
#[cfg(not(test))]
pub mod assembly;

mod uart;
