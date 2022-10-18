use std::fmt::Display;

use fake::{Dummy, Fake};
use serde::Serialize;

#[derive(Serialize, Dummy)]
pub struct TransportLog {
    pub _transport: String,
    pub syslog_facility: u8,
    pub _boot_id: String,
    pub message: String,
    pub syslog_identifier: String,
    pub __realtime_timestamp: u16,
    pub _hostname: String,
    pub _source_monotonic_timestamp: u8,
    pub _machine_id: String,
    pub __monotonic_timestamp: u8,
    pub __cursor: String,
    pub priority: u8,
}

#[derive(Serialize, Dummy)]
pub struct ExtendedTransportLog {
    pub _audit_field_scontext: String,
    pub _audit_field_permissive: u8,
    pub _source_realtime_timestamp: u16,
    pub _audit_field_tclass: String,
    pub _audit_type_name: String,
    pub _audit_field_tcontext: String,
    pub _audit_type: u8,
    pub _audit_id: u8,
    pub _pid: u8,
    pub _transport: String,
    pub syslog_facility: u8,
    pub _boot_id: String,
    pub message: String,
    pub syslog_identifier: String,
    pub __realtime_timestamp: u16,
    pub _hostname: String,
    pub _source_monotonic_timestamp: u8,
    pub _machine_id: String,
    pub __monotonic_timestamp: u8,
    pub __cursor: String,
    pub priority: u8,
    pub _comm: String,
    pub _audit_field_dev: String,
    pub _audit_field_ino: String,
    pub _audit_field_path: String,
}
