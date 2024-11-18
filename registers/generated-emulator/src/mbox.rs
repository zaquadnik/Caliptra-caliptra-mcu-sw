// Licensed under the Apache-2.0 license.
//
// generated by registers_generator with caliptra-ss repo at a621fff9df7015821eda6f7f73265fef74a01375
//
#[allow(unused_imports)]
use tock_registers::interfaces::{Readable, Writeable};
pub trait MboxPeripheral {
    fn poll(&mut self) {}
    fn warm_reset(&mut self) {}
    fn update_reset(&mut self) {}
    fn read_lock(
        &mut self,
    ) -> emulator_bus::ReadWriteRegister<u32, registers_generated::mbox::bits::Lock::Register> {
        emulator_bus::ReadWriteRegister::new(0)
    }
    fn write_lock(
        &mut self,
        _val: emulator_bus::ReadWriteRegister<u32, registers_generated::mbox::bits::Lock::Register>,
    ) {
    }
    fn read_id(&mut self) -> u32 {
        0
    }
    fn write_id(&mut self, _val: u32) {}
    fn read_cmd(&mut self) -> u32 {
        0
    }
    fn write_cmd(&mut self, _val: u32) {}
    fn read_dlen(&mut self) -> u32 {
        0
    }
    fn write_dlen(&mut self, _val: u32) {}
    fn read_datain(&mut self) -> u32 {
        0
    }
    fn write_datain(&mut self, _val: u32) {}
    fn read_dataout(&mut self) -> u32 {
        0
    }
    fn write_dataout(&mut self, _val: u32) {}
    fn read_execute(
        &mut self,
    ) -> emulator_bus::ReadWriteRegister<u32, registers_generated::mbox::bits::Execute::Register>
    {
        emulator_bus::ReadWriteRegister::new(0)
    }
    fn write_execute(
        &mut self,
        _val: emulator_bus::ReadWriteRegister<
            u32,
            registers_generated::mbox::bits::Execute::Register,
        >,
    ) {
    }
    fn read_status(
        &mut self,
    ) -> emulator_bus::ReadWriteRegister<u32, registers_generated::mbox::bits::Status::Register>
    {
        emulator_bus::ReadWriteRegister::new(0)
    }
    fn write_status(
        &mut self,
        _val: emulator_bus::ReadWriteRegister<
            u32,
            registers_generated::mbox::bits::Status::Register,
        >,
    ) {
    }
    fn read_unlock(
        &mut self,
    ) -> emulator_bus::ReadWriteRegister<u32, registers_generated::mbox::bits::Unlock::Register>
    {
        emulator_bus::ReadWriteRegister::new(0)
    }
    fn write_unlock(
        &mut self,
        _val: emulator_bus::ReadWriteRegister<
            u32,
            registers_generated::mbox::bits::Unlock::Register,
        >,
    ) {
    }
}
pub struct MboxBus {
    pub periph: Box<dyn MboxPeripheral>,
}
impl emulator_bus::Bus for MboxBus {
    fn read(
        &mut self,
        size: emulator_types::RvSize,
        addr: emulator_types::RvAddr,
    ) -> Result<emulator_types::RvData, emulator_bus::BusError> {
        match (size, addr) {
            (emulator_types::RvSize::Word, 0) => Ok(emulator_types::RvData::from(
                self.periph.read_lock().reg.get(),
            )),
            (emulator_types::RvSize::Word, 1..=3) => {
                Err(emulator_bus::BusError::LoadAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 4) => {
                Ok(emulator_types::RvData::from(self.periph.read_id()))
            }
            (emulator_types::RvSize::Word, 5..=7) => {
                Err(emulator_bus::BusError::LoadAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 8) => {
                Ok(emulator_types::RvData::from(self.periph.read_cmd()))
            }
            (emulator_types::RvSize::Word, 9..=0xb) => {
                Err(emulator_bus::BusError::LoadAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0xc) => {
                Ok(emulator_types::RvData::from(self.periph.read_dlen()))
            }
            (emulator_types::RvSize::Word, 0xd..=0xf) => {
                Err(emulator_bus::BusError::LoadAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0x10) => {
                Ok(emulator_types::RvData::from(self.periph.read_datain()))
            }
            (emulator_types::RvSize::Word, 0x11..=0x13) => {
                Err(emulator_bus::BusError::LoadAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0x14) => {
                Ok(emulator_types::RvData::from(self.periph.read_dataout()))
            }
            (emulator_types::RvSize::Word, 0x15..=0x17) => {
                Err(emulator_bus::BusError::LoadAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0x18) => Ok(emulator_types::RvData::from(
                self.periph.read_execute().reg.get(),
            )),
            (emulator_types::RvSize::Word, 0x19..=0x1b) => {
                Err(emulator_bus::BusError::LoadAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0x1c) => Ok(emulator_types::RvData::from(
                self.periph.read_status().reg.get(),
            )),
            (emulator_types::RvSize::Word, 0x1d..=0x1f) => {
                Err(emulator_bus::BusError::LoadAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0x20) => Ok(emulator_types::RvData::from(
                self.periph.read_unlock().reg.get(),
            )),
            (emulator_types::RvSize::Word, 0x21..=0x23) => {
                Err(emulator_bus::BusError::LoadAddrMisaligned)
            }
            _ => Err(emulator_bus::BusError::LoadAccessFault),
        }
    }
    fn write(
        &mut self,
        size: emulator_types::RvSize,
        addr: emulator_types::RvAddr,
        val: emulator_types::RvData,
    ) -> Result<(), emulator_bus::BusError> {
        match (size, addr) {
            (emulator_types::RvSize::Word, 0) => {
                self.periph
                    .write_lock(emulator_bus::ReadWriteRegister::new(val));
                Ok(())
            }
            (emulator_types::RvSize::Word, 1..=3) => {
                Err(emulator_bus::BusError::StoreAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 8) => {
                self.periph.write_cmd(val);
                Ok(())
            }
            (emulator_types::RvSize::Word, 9..=0xb) => {
                Err(emulator_bus::BusError::StoreAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0xc) => {
                self.periph.write_dlen(val);
                Ok(())
            }
            (emulator_types::RvSize::Word, 0xd..=0xf) => {
                Err(emulator_bus::BusError::StoreAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0x10) => {
                self.periph.write_datain(val);
                Ok(())
            }
            (emulator_types::RvSize::Word, 0x11..=0x13) => {
                Err(emulator_bus::BusError::StoreAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0x14) => {
                self.periph.write_dataout(val);
                Ok(())
            }
            (emulator_types::RvSize::Word, 0x15..=0x17) => {
                Err(emulator_bus::BusError::StoreAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0x18) => {
                self.periph
                    .write_execute(emulator_bus::ReadWriteRegister::new(val));
                Ok(())
            }
            (emulator_types::RvSize::Word, 0x19..=0x1b) => {
                Err(emulator_bus::BusError::StoreAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0x1c) => {
                self.periph
                    .write_status(emulator_bus::ReadWriteRegister::new(val));
                Ok(())
            }
            (emulator_types::RvSize::Word, 0x1d..=0x1f) => {
                Err(emulator_bus::BusError::StoreAddrMisaligned)
            }
            (emulator_types::RvSize::Word, 0x20) => {
                self.periph
                    .write_unlock(emulator_bus::ReadWriteRegister::new(val));
                Ok(())
            }
            (emulator_types::RvSize::Word, 0x21..=0x23) => {
                Err(emulator_bus::BusError::StoreAddrMisaligned)
            }
            _ => Err(emulator_bus::BusError::StoreAccessFault),
        }
    }
    fn poll(&mut self) {
        self.periph.poll();
    }
    fn warm_reset(&mut self) {
        self.periph.warm_reset();
    }
    fn update_reset(&mut self) {
        self.periph.update_reset();
    }
}
