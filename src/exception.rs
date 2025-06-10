
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionCode {
    /// The function code received in the query is not an allowable 
    /// action for the server. This may be because the function code 
    /// is only applicable to newer devices, and was not implemented 
    /// in the unit selected. It could also indicate that the server 
    /// is in the wrong state to process a request of this type, for 
    /// example because it is unconfigured and is being asked to return 
    /// register values.
    IllegalFunction = 0x01,

    /// The data address received in the query is not an allowable 
    /// address for the server. More specifically, the combination 
    /// of reference number and transfer length is invalid. For a 
    /// controller with 100 registers, the PDU addresses the first 
    /// register as 0, and the last one as 99. If a request is submitted 
    /// with a starting register address of 96 and a quantity of registers 
    /// of 4, then this request will successfully operate (address-wise at least) 
    /// on registers 96, 97, 98, 99. If a request is submitted with a starting 
    /// register address of 96 and a quantity of registers of 5, then this 
    /// request will fail with Exception Code 0x02 “Illegal Data Address” 
    /// since it attempts to operate on registers 96, 97, 98, 99 and 100, and 
    /// there is no register with address 100.
    IllegalDataAddress = 0x02,

    /// A value contained in the query data field is not an allowable 
    /// value for server. This indicates a fault in the structure of the 
    /// remainder of a complex request, such as that the implied length is 
    /// incorrect. It specifically does NOT mean that a data item submitted 
    /// for storage in a register has a value outside the expectation of the 
    /// application program, since the MODBUS protocol is unaware of the 
    /// significance of any particular value of any particular register.
    IllegalDataValue = 0x03,

    /// An unrecoverable error occurred while the server was attempting 
    /// to perform the requested action.
    SlaveDeviceFailure = 0x04,

    /// Specialized use in conjunction with programming commands.
    /// The server has accepted the request and is processing it, but 
    /// a long duration of time will be required to do so. This response 
    /// is returned to prevent a timeout error from occurring in the client. 
    /// The client can next issue a Poll Program Complete message to 
    /// determine if processing is completed.
    Acknowledge = 0x05,

    /// Specialized use in conjunction with programming commands.
    /// The server is engaged in processing a long–duration program command. 
    /// The client should retransmit the message later when the server is free.
    SlaveDeviceBusy = 0x06,

    /// Specialized use in conjunction with function codes 20 and 21 and reference type 6, to 
    /// indicate that the extended file area failed to pass a consistency check.
    /// The server attempted to read record file, but detected a parity error in the memory. 
    /// The client can retry the request, but service may be required on the server device.
    MemoryParityError = 0x08,

    /// Specialized use in conjunction with gateways, indicates that 
    /// the gateway was unable to allocate an internal communication
    /// path from the input port to the output port for processing 
    /// the request. Usually means that the gateway is misconfigured or overloaded.
    GatewayPathUnavailable = 0x0A,

    /// Specialized use in conjunction with gateways, indicates that 
    /// no response was obtained from the target device. Usually 
    /// means that the device is not present on the network.
    GatewayTargetFailedToRespond = 0x0B,

    /// Set if nothing else is applicable.
    Unknown(u8),
}

impl From<ExceptionCode> for u8 {
    fn from(value: ExceptionCode) -> Self {
        match value {
            ExceptionCode::IllegalFunction => 0x01,
            ExceptionCode::IllegalDataAddress => 0x02,
            ExceptionCode::IllegalDataValue => 0x03,
            ExceptionCode::SlaveDeviceFailure => 0x04,
            ExceptionCode::Acknowledge => 0x05,
            ExceptionCode::SlaveDeviceBusy => 0x06,
            ExceptionCode::MemoryParityError => 0x08,
            ExceptionCode::GatewayPathUnavailable => 0x0A,
            ExceptionCode::GatewayTargetFailedToRespond => 0x0B,
            ExceptionCode::Unknown(value) => value,
        }
    }
}

impl From<u8> for ExceptionCode {
    fn from(value: u8) -> Self {
        match value {
            0x01 => ExceptionCode::IllegalFunction,
            0x02 => ExceptionCode::IllegalDataAddress,
            0x03 => ExceptionCode::IllegalDataValue,
            0x04 => ExceptionCode::SlaveDeviceFailure,
            0x05 => ExceptionCode::Acknowledge,
            0x06 => ExceptionCode::SlaveDeviceBusy,
            0x08 => ExceptionCode::MemoryParityError,
            0x0A => ExceptionCode::GatewayPathUnavailable,
            0x0B => ExceptionCode::GatewayTargetFailedToRespond,
            _ => ExceptionCode::Unknown(value),
        }
    }
}

#[cfg(test)]
mod exeception_tests {
    use super::*;

    #[test]
    fn test_from_001() {
        let exception = ExceptionCode::IllegalFunction;
        let value = u8::from(exception);
        let new_exception = ExceptionCode::from(value);
        assert_eq!(exception, new_exception);
    }

    #[test]
    fn test_from_002() {
        let exception = ExceptionCode::IllegalDataAddress;
        let value = u8::from(exception);
        let new_exception = ExceptionCode::from(value);
        assert_eq!(exception, new_exception);
    }

    #[test]
    fn test_from_003() {
        let exception = ExceptionCode::IllegalDataValue;
        let value = u8::from(exception);
        let new_exception = ExceptionCode::from(value);
        assert_eq!(exception, new_exception);
    }

    #[test]
    fn test_from_004() {
        let exception = ExceptionCode::SlaveDeviceFailure;
        let value = u8::from(exception);
        let new_exception = ExceptionCode::from(value);
        assert_eq!(exception, new_exception);
    }

    #[test]
    fn test_from_005() {
        let exception = ExceptionCode::Acknowledge;
        let value = u8::from(exception);
        let new_exception = ExceptionCode::from(value);
        assert_eq!(exception, new_exception);
    }

    #[test]
    fn test_from_006() {
        let exception = ExceptionCode::SlaveDeviceBusy;
        let value = u8::from(exception);
        let new_exception = ExceptionCode::from(value);
        assert_eq!(exception, new_exception);
    }

    #[test]
    fn test_from_007() {
        let exception = ExceptionCode::MemoryParityError;
        let value = u8::from(exception);
        let new_exception = ExceptionCode::from(value);
        assert_eq!(exception, new_exception);
    }

    #[test]
    fn test_from_008() {
        let exception = ExceptionCode::GatewayPathUnavailable;
        let value = u8::from(exception);
        let new_exception = ExceptionCode::from(value);
        assert_eq!(exception, new_exception);
    }

    #[test]
    fn test_from_009() {
        let exception = ExceptionCode::GatewayTargetFailedToRespond;
        let value = u8::from(exception);
        let new_exception = ExceptionCode::from(value);
        assert_eq!(exception, new_exception);
    }

    #[test]
    fn test_from_010() {
        for v in 0x0C..0xFF {
            let exception = ExceptionCode::Unknown(v);
            let value = u8::from(exception);
            let new_exception = ExceptionCode::from(value);
            assert_eq!(exception, new_exception);
        }
    }

}