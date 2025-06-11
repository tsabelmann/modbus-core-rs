use std::{io::{Read, Write}, net::TcpListener};
use modbus_stack::{decode::request::{ReadHoldingRegistersRequestDecoder, WriteMultipleRegistersRequestDecoder, WriteSingleRegisterRequestDecoder}, encode::{response::ReadHoldingRegistersReponseEncoder}, tcp::{ModbusTcpFrame, ModbusTcpFrameDecoder}, ExceptionCode, PduData};
use modbus_stack::constants::MODBUS_TCP_FRAME_DATA_LENTGH;

// fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0u8; 1024];
//     if let Ok(length)  = stream.read(&mut buffer) {
//         let slice = &buffer[0..length];
//         if let Ok(string) = std::str::from_utf8(slice) {
//             println!("string={}", string);
//         }
//     }
// }

fn main() -> std::io::Result<()> {
    let mut buffer = [0u8; MODBUS_TCP_FRAME_DATA_LENTGH];
    let mut out_buffer = [0u8; MODBUS_TCP_FRAME_DATA_LENTGH];
    let mut decoder = ModbusTcpFrameDecoder::new();


    let listener = TcpListener::bind("0.0.0.0:1502")?;

    // accept connections and process them serially
    let socket = listener.accept();
    let mut stream = match socket {
        Ok((stream, _)) => {
                stream    
            },
        _ => return Ok(())
    };

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
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


                        if let Ok(read_holding_register_request) = ReadHoldingRegistersRequestDecoder::new(&frame) {
                            println!("Starting-Address: {:04X}", read_holding_register_request.starting_address());
                            println!("Quantity-Of-Register: {}", read_holding_register_request.quantity_of_registers());

                            let mut out_frame = unsafe {
                                ModbusTcpFrame::new_unchecked(&mut out_buffer)
                            };
                            frame.copy_to(&mut out_frame);
                            
                            let string = "SunS";
                            let bytes = string.as_bytes();
                            let upper = u16::from_be_bytes([bytes[0], bytes[1]]);
                            let lower = u16::from_be_bytes([bytes[2], bytes[3]]);

                            let quantity_of_registers = read_holding_register_request.quantity_of_registers();
                            match ReadHoldingRegistersReponseEncoder::encode(&mut out_frame, quantity_of_registers, [upper, lower].into_iter()) {
                                Ok(_) => {
                                    println!("Success!");
                                    let length = (out_frame.length() + 6) as usize;
                                    let result = stream.write(&out_buffer[..length]);
                                    println!("Written-Data: {:?}", result);
                                    println!("Bytes: {:?}", &out_buffer[..length]);
                                },
                                Err(reason) => println!("Not a success - Why? {:?}", reason)
                            };

                            // match ReadHoldingRegistersReponseEncoder::encode_exception(&mut out_frame, ExceptionCode::IllegalDataAddress) {
                            //     Ok(_) => {
                            //         println!("Success!");
                            //         let length = (out_frame.length() + 6) as usize;
                            //         let result = stream.write(&out_buffer[..length]);
                            //         println!("Written-Data: {:?}", result);
                            //         println!("Bytes: {:?}", &out_buffer[..length]);
                            //     },
                            //     Err(reason) => println!("Not a success - Why? {:?}", reason)
                            // };

                        }
                        // if let Ok(write_single_request) = WriteSingleRegisterRequest::new(&frame) {
                        //     println!("Write-Single-Register");
                        //     println!("Register-Address: {:04X}", write_single_request.register_address());
                        //     println!("Register-Value: {:04X}", write_single_request.register_value());
                        // }
                    }
                }
            },
            Err(_) => {}
        };
    }

    Ok(())
}