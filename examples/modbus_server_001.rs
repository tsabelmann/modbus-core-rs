use std::{io::{Read, Write}, net::TcpListener};
use modbus_stack::{decode::request::{ReadHoldingRegistersRequestDecoder, WriteMultipleRegistersRequestDecoder, WriteSingleRegisterRequestDecoder}, encode::response::{ReadHoldingRegistersReponseEncoder, WriteSingleRegisterReponseEncoder}, register::{IntoRegIter, RegU16, RegU16Array, RegU32, RegU64}, tcp::{ModbusTcpFrame, ModbusTcpFrameDecoder}, ExceptionCode, PduData};
use modbus_stack::constants::MODBUS_TCP_FRAME_DATA_LENTGH;
use modbus_stack::register::RO;

// fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0u8; 1024];
//     if let Ok(length)  = stream.read(&mut buffer) {
//         let slice = &buffer[0..length];
//         if let Ok(string) = std::str::from_utf8(slice) {
//             println!("string={}", string);
//         }
//     }
// }

pub const SUN_S: RO::<RegU32> = RO::<RegU32>::new(RegU32::new(0x53_75_6E_53));  
pub const MODEL_1_ID: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(1));
pub const MODEL_1_LENGTH: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(68));
pub const MODEL_1_MANUFACTURER: RO::<RegU16Array<16>> = RO::<RegU16Array<16>>::new(RegU16Array::from_str("NAEXT GmbH"));
pub const MODEL_1_MODEL: RO::<RegU16Array<16>> = RO::<RegU16Array<16>>::new(RegU16Array::from_str("MultiBMS"));
pub const MODEL_1_OPTIONS: RO::<RegU16Array<8>> = RO::<RegU16Array<8>>::new(RegU16Array::from_str("NOT"));
pub const MODEL_1_VERSION: RO::<RegU16Array<8>> = RO::<RegU16Array<8>>::new(RegU16Array::from_str("0.1.0-test"));
pub const MODEL_1_SERIAL: RO::<RegU16Array<16>> = RO::<RegU16Array<16>>::new(RegU16Array::from_str("SunSpec funktioniert!"));
pub const MODEL_1_DEVICE_ADDRESS: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(0));
pub const MODEL_1_PAD: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(0));

pub const MODEL_END: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(0xFFFF));
pub const END: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(0));

pub fn dispatch(starting_address: u16, quantity_of_registers: u16) -> Option<impl Iterator<Item = &'static u16>> {
    if starting_address < 40000 || starting_address > 40074 {
        return None;
    }

    let diff = starting_address - 40000;
    if diff + quantity_of_registers > 74 {
        return None;
    }

    // Beginne mit dem ersten Iterator
    let iter = SUN_S.reg_iter();

    let model_1_id_iter = MODEL_1_ID.reg_iter();
    let model_1_length_iter = MODEL_1_LENGTH.reg_iter();
    let model_1_manufacturer_iter = MODEL_1_MANUFACTURER.reg_iter();
    let model_1_model_iter = MODEL_1_MODEL.reg_iter();
    let model_1_options_iter = MODEL_1_OPTIONS.reg_iter();
    let model_1_version_iter = MODEL_1_VERSION.reg_iter();
    let model_1_serial_iter = MODEL_1_SERIAL.reg_iter();
    let model_1_device_address_iter = MODEL_1_DEVICE_ADDRESS.reg_iter();
    let model_1_pad_iter = MODEL_1_PAD.reg_iter();
    let iter = iter
        .chain(model_1_id_iter)
        .chain(model_1_length_iter)
        .chain(model_1_manufacturer_iter)
        .chain(model_1_model_iter)
        .chain(model_1_options_iter)
        .chain(model_1_version_iter)
        .chain(model_1_serial_iter)
        .chain(model_1_device_address_iter)
        .chain(model_1_pad_iter);

    // Hänge den letzten an
    let iter = iter.chain(MODEL_END.reg_iter()).chain(END.reg_iter());
    let iter = iter.skip(diff as usize).take(quantity_of_registers as usize);

    println!("SIZE={}", size_of_val(&iter));
    Some(iter)
}

fn main() -> std::io::Result<()> {
    let mut buffer = [0u8; MODBUS_TCP_FRAME_DATA_LENTGH];
    let mut out_buffer = [0u8; MODBUS_TCP_FRAME_DATA_LENTGH];
    let mut decoder = ModbusTcpFrameDecoder::new();
    let listener = TcpListener::bind("0.0.0.0:1502")?;

    // // accept connections and process them serially
    // let socket = listener.accept();
    // let mut stream = match socket {
    //     Ok((stream, _)) => {
    //             stream    
    //         },
    //     _ => return Ok(())
    // };

    for incoming in listener.incoming() {
        let mut stream = match incoming {
            Ok(stream) => {
                    stream    
                },
            _ => continue
        };

        println!("Socket created!");
        'main: loop {
            match stream.read(&mut buffer) {
            Ok(size) => {
                println!("Size: {:?}", size);
                println!("Bytes: {:?}", &out_buffer[..size]);
                for val in &buffer[..size] {
                    if let Ok(mut frame) = decoder.push_data(*val) {
                        println!("Frame");
                        println!("Transation-Identifier: {}", frame.transaction_identifier());
                        println!("Protocol-Identifier: {}", frame.protocol_identifier());
                        println!("Length: {}", frame.length());
                        println!("Unit-Identifier: {}", frame.unit_identifier());
                        println!("Function-Code: {:?}", frame.function_code());

                        // match WriteMultipleRegistersRequestDecoder::new(&frame) {
                        //     Ok(write_multiple_registers_request) => {
                        //         println!("Write-Multiple-Registers");
                        //         println!("Starting-Address: {:04X}", write_multiple_registers_request.starting_address());
                        //         println!("Quantity-Of-Register: {}", write_multiple_registers_request.quantity_of_registers());

                        //         for value in write_multiple_registers_request {
                        //             println!("Register={:04X}", value);
                        //         }
                        //     },
                        //     Err(reason) => println!("Not a success - Why? {:?}", reason)
                        // };


                        if let Ok(req) = ReadHoldingRegistersRequestDecoder::new(&frame) {
                            println!("Starting-Address: {:04X}", req.starting_address());
                            println!("Quantity-Of-Register: {}", req.quantity_of_registers());

                            let mut out_frame = unsafe {
                                ModbusTcpFrame::new_unchecked(&mut out_buffer)
                            };
                            frame.copy_to(&mut out_frame);
                            
                            let starting_address = req.starting_address();
                            let quantity_of_registers = req.quantity_of_registers();
                            
                            if let Some(iter) = dispatch(starting_address, quantity_of_registers) {
                                match ReadHoldingRegistersReponseEncoder::encode(&mut out_frame, quantity_of_registers, iter) {
                                    Ok(_) => {
                                        println!("Response!");
                                        let length = (out_frame.length() + 6) as usize;
                                        let result = stream.write(&out_buffer[..length]);
                                        println!("Written-Data: {:?}", result);
                                        println!("Bytes: {:?}", &out_buffer[..length]);
                                    },
                                    Err(reason) => println!("Not a success - Why? {:?}", reason)
                                };
                            } else {
                                match ReadHoldingRegistersReponseEncoder::encode_exception(&mut out_frame, ExceptionCode::IllegalDataAddress) {
                                    Ok(_) => {
                                        println!("Exception!");
                                        let length = (out_frame.length() + 6) as usize;
                                        let result = stream.write(&out_buffer[..length]);
                                        println!("Written-Data: {:?}", result);
                                        println!("Bytes: {:?}", &out_buffer[..length]);
                                    },
                                    Err(reason) => println!("Not a success - Why? {:?}", reason)
                                };
                            }
                        } else {
                            // match ReadHoldingRegistersReponseEncoder::encode_exception(&mut out_frame, ExceptionCode::IllegalDataAddress) {
                            //     Ok(_) => {
                            //         println!("Exception!");
                            //         let length = (out_frame.length() + 6) as usize;
                            //         let result = stream.write(&out_buffer[..length]);
                            //         println!("Written-Data: {:?}", result);
                            //         println!("Bytes: {:?}", &out_buffer[..length]);
                            //     },
                            //     Err(reason) => println!("Not a success - Why? {:?}", reason)
                            // };
                        }

                        // if let Ok(request) = WriteSingleRegisterRequestDecoder::new(&frame) {
                        //     println!("Write-Single-Register");
                        //     println!("Register-Address: {:04X}", request.register_address());
                        //     println!("Register-Value: {:04X}", request.register_value());

                        //     let mut out_frame = unsafe {
                        //         ModbusTcpFrame::new_unchecked(&mut out_buffer)
                        //     };
                        //     frame.copy_to(&mut out_frame);

                        //     match WriteSingleRegisterReponseEncoder::encode(&mut out_frame, request.register_address(), request.register_value()) {
                        //         Ok(_) => {
                        //             println!("Success!");
                        //             let length = (out_frame.length() + 6) as usize;
                        //             let result = stream.write(&out_buffer[..length]);
                        //             println!("Written-Data: {:?}", result);
                        //             println!("Bytes: {:?}", &out_buffer[..length]);
                        //         },
                        //         Err(reason) => println!("Not a success - Why? {:?}", reason)
                        //     };
                        // }
                    }
                }
            },
            Err(err) => println!("Stream-Error: {:?}", err)
        }
        let mut test_buffer = [];
        match stream.read(&mut test_buffer) {
            Ok(0) => {},
            _ => {
                println!("Destruct Socket");
                break 'main;
            }
        };
    
    }
}
    Ok(())
}