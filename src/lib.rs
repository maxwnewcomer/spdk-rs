// the improper_ctypes is needed as because
// spdk_nvme_ctrlr_data is 128 bit

#![allow(
    clippy::all,
    elided_lifetimes_in_paths,
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unknown_lints,
    unused,
    clippy::upper_case_acronyms
)]

use std::os::raw::c_char;

#[macro_use]
extern crate tracing;
extern crate serde;
extern crate serde_json;

pub mod bdevs;
pub mod cpu_cores;
pub mod ffihelper;
pub mod libspdk;

mod bdev;
mod bdev_async;
mod bdev_builder;
mod bdev_desc;
mod bdev_io;
mod bdev_iter;
mod bdev_module;
mod bdev_ops;
mod dma;
mod error;
mod io_channel;
mod io_device_traverse;
mod io_devices;
mod io_type;
mod io_vec;
mod json_write_context;
mod nvme;
mod nvmf;
mod poller;
mod thread;
mod unsafe_types;
mod untyped_bdev;
mod uuid;

pub use crate::{
    bdev::Bdev,
    bdev_async::{BdevAsyncCallContext, BdevStats},
    bdev_builder::BdevBuilder,
    bdev_desc::{BdevDesc, BdevDescError, BdevEvent, LbaRange, LbaRangeLock},
    bdev_io::BdevIo,
    bdev_iter::{BdevGlobalIter, BdevModuleIter},
    bdev_module::{
        BdevModule, BdevModuleBuild, BdevModuleBuilder, WithModuleConfigJson, WithModuleFini,
        WithModuleGetCtxSize, WithModuleInit,
    },
    bdev_ops::BdevOps,
    cpu_cores::{Core, CoreIterator, Cores, RoundRobinCoreSelector},
    dma::{DmaBuf, DmaError},
    error::{spdk_error, SpdkError, SpdkResult},
    io_channel::{IoChannel, IoChannelGuard},
    io_device_traverse::{ChannelTraverseStatus, IoDeviceChannelTraverse},
    io_devices::IoDevice,
    io_type::{IoStatus, IoType},
    io_vec::{AsIoVecPtr, AsIoVecs, IoVec},
    json_write_context::JsonWriteContext,
    nvme::{
        nvme_admin_opc, nvme_nvm_opcode, nvme_reservation_acquire_action,
        nvme_reservation_register_action, nvme_reservation_register_cptpl, nvme_reservation_type,
        NvmeStatus,
    },
    nvmf::{NvmfController, NvmfSubsystemEvent},
    poller::{Poller, PollerBuilder},
    thread::{CurrentThreadGuard, Thread},
    unsafe_types::{UnsafeData, UnsafeRef},
    untyped_bdev::UntypedBdev,
    uuid::Uuid,
};

/// TODO
pub type LogProto = Option<
    extern "C" fn(
        level: i32,
        file: *const c_char,
        line: u32,
        func: *const c_char,
        buf: *const c_char,
        n: i32,
    ),
>;

#[cfg(target_arch = "x86_64")]
extern "C" {
    /// TODO
    pub fn spdk_rs_log(
        level: i32,
        file: *const c_char,
        line: i32,
        func: *const c_char,
        format: *const c_char,
        args: *mut libspdk::__va_list_tag,
    );

    /// TODO
    pub static mut logfn: LogProto;
}

#[cfg(target_arch = "aarch64")]
extern "C" {
    /// TODO
    pub fn spdk_rs_log(
        level: i32,
        file: *const c_char,
        line: i32,
        func: *const c_char,
        format: *const c_char,
        args: libspdk::va_list,
    );

    /// TODO
    pub static mut logfn: LogProto;
}
