// Binary readers for common types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl LoginRequestHeader {
    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        let client_version = read_string(reader)?;
        let length = read_u32(reader)?;
        let auth_type = NetAuthType::try_from(read_u32(reader)?)?;
        let flags = AuthFlags::try_from(read_u32(reader)?)?;
        let sequence = read_u32(reader)?;
        let account = read_string(reader)?;
        let account_to_login_as = read_string(reader)?;

        match auth_type {
            NetAuthType::AccountPassword => {
                let password = read_string(reader)?;

                Ok(Self::Type00000002 {
                    client_version,
                    length,
                    flags,
                    sequence,
                    account,
                    account_to_login_as,
                    password,
                })
            }
            NetAuthType::GlsTicket => {
                let gls_ticket = read_string(reader)?;

                Ok(Self::Type40000002 {
                    client_version,
                    length,
                    flags,
                    sequence,
                    account,
                    account_to_login_as,
                    gls_ticket,
                })
            }
            _ => Err(format!("Unknown AuthType value: {:#x}", auth_type as u32).into())
        }
    }
}

