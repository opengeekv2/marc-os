#[cfg(test)]
use std::fmt;
#[cfg(test)]
use mockall_double::double;

pub mod internal {
    #[cfg(test)]
    use mockall::automock;
    use volatile_register::{RW, RO};

    #[repr(C)]
    pub struct UARTRegisters {
        pub rthr_dll: RW<u8>,
        pub ier_dlm: RW<u8>,
        pub isr_fcr: RW<u8>,
        pub lcr_pd: RW<u8>,
        pub mcr: RW<u8>,
        pub lsr: RO<u8>,
        pub msr: RO<u8>,
        pub spr: RO<u8>,
    }

    pub struct UART {
        uart_registers: &'static mut UARTRegisters
    }

    #[cfg_attr(test, automock)]
    impl UART {
        pub fn new(uart_address: usize) -> Self {
            UART {
                uart_registers: unsafe { &mut *(uart_address as *mut UARTRegisters) }
            }
        }
        pub fn get_word_length(&self) -> u8 {
            unsafe { 0b00000011 & self.uart_registers.lcr_pd.read() }
        }
        pub fn set_word_length(&self, word_length: u8) {
            return unsafe {
                self.uart_registers.lcr_pd.write(self.get_word_length() & 0b11111100 | word_length) };
        }
        pub fn enable_fifo(&self) {
            return unsafe { self.uart_registers.isr_fcr.write(0b1) };
        }
        pub fn enable_receiver_buffer_interrupts(&self) {
            return unsafe { self.uart_registers.ier_dlm.write(0b1) };
        }
        pub fn enable_divisor_latch_access_bit(&self) {
            return unsafe {
                self.uart_registers.lcr_pd.write(self.uart_registers.lcr_pd.read() | 0b10000000 ) };
        }
        pub fn disable_divisor_latch_access_bit(&self) {
            return unsafe {
                self.uart_registers.lcr_pd.write(self.uart_registers.lcr_pd.read() & 0b01111111 ) };
        }
        pub fn set_divisor_least(&self, divisor_least: u8) {
            return unsafe {
                self.uart_registers.rthr_dll.write(divisor_least) };
        }
        pub fn set_divisor_most(&self, divisor_most: u8) {
            return unsafe {
                self.uart_registers.ier_dlm.write(divisor_most) };
        }
        pub fn set_thr(&self, c: u8) {
            unsafe { self.uart_registers.rthr_dll.write(c) };
        }
        pub fn get_dr(&self) -> u8 {
            unsafe { 0b00000001 & self.uart_registers.lsr.read() }
        } 
        pub fn get_rhr(&self) -> u8 {
            unsafe { self.uart_registers.rthr_dll.read() }
        }
    }
}

#[cfg_attr(test, double)]
use internal::UART;

use core::fmt::{Error, Write};

pub struct UARTDriver {
    uart: UART
}

impl UARTDriver {
    pub fn new(uart: UART) -> Self {
        uart.set_word_length(0b11);
        uart.enable_fifo();
        uart.enable_receiver_buffer_interrupts();
        uart.enable_divisor_latch_access_bit();
        let divisor: u16 = 592;
        let divisor_least: u8 = (divisor & 0xff).try_into().unwrap();
        let divisor_most: u8 = (divisor >> 8).try_into().unwrap();
        uart.set_divisor_least(divisor_least);
        uart.set_divisor_most(divisor_most);
        uart.disable_divisor_latch_access_bit();
        UARTDriver {
            uart
        }
    }
    fn put(&self, c: u8) {
        self.uart.set_thr(c);
    }
    pub fn get(&self) -> Option<u8> {
        if self.uart.get_dr() == 0b00000001 {
            Some(self.uart.get_rhr())
        } else {
            None
        }
    }
}

impl Write for UARTDriver {
    fn write_str(&mut self, s: &str) -> Result<(),Error> {
        for c in s.bytes() {
            self.put(c);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    fn setup_uart_expectations(uart: &mut UART) {
        uart.expect_set_word_length().with(eq(0b11)).times(1).return_const(());
        uart.expect_enable_fifo().times(1).return_const(());
        uart.expect_enable_receiver_buffer_interrupts().times(1).return_const(());
        uart.expect_enable_divisor_latch_access_bit().times(1).return_const(());
        uart.expect_set_divisor_least().with(eq::<u8>((592 & 0xff).try_into().unwrap())).times(1).return_const(());
        uart.expect_set_divisor_most().with(eq::<u8>((592 >> 8).try_into().unwrap())).times(1).return_const(());
        uart.expect_disable_divisor_latch_access_bit().times(1).return_const(());
    }

    #[test]
    fn it_should_put_on_thr_on_write() {
        let mut uart = UART::default();
        setup_uart_expectations(&mut uart);
        uart.expect_set_thr().with(eq('c' as u8)).times(1).return_const(());
        let mut uart_writer = UARTDriver::new(uart);
        uart_writer.write_str("c");
    }

    #[test]
    fn it_should_put_thr_on_put() {
        let mut uart = UART::default();
        setup_uart_expectations(&mut uart);
        uart.expect_set_thr().with(eq('c' as u8)).times(1).return_const(());
        let mut uart_driver = UARTDriver::new(uart);
        uart_driver.put(b'c');
    }

    #[test]
    fn it_should_get_none_if_dr_is_0() {
        let mut uart = UART::default();
        setup_uart_expectations(&mut uart);
        uart.expect_get_dr().times(1).return_const(0);
        uart.expect_get_rhr().times(0);
        let mut uart_driver = UARTDriver::new(uart);
        assert!(uart_driver.get().is_none());
    }

    #[test]
    fn it_should_get_rhr_if_dr_is_1() {
        let mut uart = UART::default();
        setup_uart_expectations(&mut uart);
        uart.expect_get_dr().times(1).return_const(1);
        uart.expect_get_rhr().times(1).return_const(b'a');
        let mut uart_driver = UARTDriver::new(uart);
        assert!(uart_driver.get() == Some(b'a'));
    }

    #[test]
    fn it_should_init_uart() {
        let mut uart = UART::default();
        setup_uart_expectations(&mut uart);
        UARTDriver::new(uart);
    }


}