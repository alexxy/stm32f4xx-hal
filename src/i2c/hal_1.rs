use embedded_hal_one::i2c::{Error, ErrorKind, ErrorType};

impl Error for super::Error {
    fn kind(&self) -> ErrorKind {
        match *self {
            Self::Overrun => ErrorKind::Overrun,
            Self::Bus => ErrorKind::Bus,
            Self::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            Self::NoAcknowledge(nack) => ErrorKind::NoAcknowledge(nack),
            Self::Crc | Self::Timeout => ErrorKind::Other,
        }
    }
}

impl<I2C: super::Instance> ErrorType for super::I2c<I2C> {
    type Error = super::Error;
}

mod blocking {
    use super::super::{I2c, Instance};
    use embedded_hal_one::i2c::blocking::Operation;

    impl<I2C: Instance> embedded_hal_one::i2c::blocking::I2c for I2c<I2C> {
        fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            self.read(addr, buffer)
        }

        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            self.write(addr, bytes)
        }

        fn write_iter<B>(&mut self, addr: u8, bytes: B) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.write_iter(addr, bytes)
        }

        fn write_read(
            &mut self,
            addr: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.write_read(addr, bytes, buffer)
        }

        fn write_iter_read<B>(
            &mut self,
            addr: u8,
            bytes: B,
            buffer: &mut [u8],
        ) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.write_iter_read(addr, bytes, buffer)
        }

        fn transaction(
            &mut self,
            addr: u8,
            operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            self.transaction_slice(addr, operations)
        }

        fn transaction_iter<'a, O>(&mut self, addr: u8, operations: O) -> Result<(), Self::Error>
        where
            O: IntoIterator<Item = Operation<'a>>,
        {
            let it = operations.into_iter();
            self.transaction(addr, it)
        }
    }
}
