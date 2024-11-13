extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use rust_dynamic::value::Value;

#[derive(Debug, Clone)]
pub enum MemOperation {
    Total,
    Free,
    Used,
    Shared,
    Buffers,
    Cached,
}

pub fn bund_mem_base(vm: &mut VM, op: StackOps, mop: MemOperation, err_prefix: String) -> Result<&mut VM, Error> {
    let mem_info = match sys_metrics::memory::get_memory() {
        Ok(mem_info) => mem_info,
        Err(err) => {
            bail!("{} returns: {}", &err_prefix, err);
        }
    };
    let res = match mop {
        MemOperation::Total => mem_info.total,
        MemOperation::Free => mem_info.free,
        MemOperation::Used => mem_info.used,
        MemOperation::Shared => mem_info.shared,
        MemOperation::Buffers => mem_info.buffers,
        MemOperation::Cached => mem_info.cached,
    };
    let res_value = Value::from_int(((res*1024)*1024) as i64);
    let _ = match op {
        StackOps::FromStack => vm.stack.push(res_value),
        StackOps::FromWorkBench => vm.stack.push_to_workbench(res_value),
    };
    Ok(vm)
}

pub fn stdlib_sysinfo_mem_total_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromStack, MemOperation::Total, "SYSINFO.MEM.TOTAL".to_string())
}
pub fn stdlib_sysinfo_mem_total_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromWorkBench, MemOperation::Total, "SYSINFO.MEM.TOTAL.".to_string())
}

pub fn stdlib_sysinfo_mem_free_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromStack, MemOperation::Free, "SYSINFO.MEM.FREE".to_string())
}
pub fn stdlib_sysinfo_mem_free_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromWorkBench, MemOperation::Free, "SYSINFO.MEM.FREE.".to_string())
}

pub fn stdlib_sysinfo_mem_used_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromStack, MemOperation::Free, "SYSINFO.MEM.USED".to_string())
}
pub fn stdlib_sysinfo_mem_used_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromWorkBench, MemOperation::Used, "SYSINFO.MEM.USED.".to_string())
}

pub fn stdlib_sysinfo_mem_shared_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromStack, MemOperation::Shared, "SYSINFO.MEM.SHARED".to_string())
}
pub fn stdlib_sysinfo_mem_shared_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromWorkBench, MemOperation::Shared, "SYSINFO.MEM.SHARED.".to_string())
}

pub fn stdlib_sysinfo_mem_buffers_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromStack, MemOperation::Buffers, "SYSINFO.MEM.BUFFERS".to_string())
}
pub fn stdlib_sysinfo_mem_buffers_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromWorkBench, MemOperation::Buffers, "SYSINFO.MEM.BUFFERS.".to_string())
}

pub fn stdlib_sysinfo_mem_cached_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromStack, MemOperation::Cached, "SYSINFO.MEM.CACHED".to_string())
}
pub fn stdlib_sysinfo_mem_cached_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_mem_base(vm, StackOps::FromWorkBench, MemOperation::Cached, "SYSINFO.MEM.CACHED.".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("sysinfo.mem.total".to_string(), stdlib_sysinfo_mem_total_stack);
    let _ = bc.vm.register_inline("sysinfo.mem.total.".to_string(), stdlib_sysinfo_mem_total_workbench);
    let _ = bc.vm.register_inline("sysinfo.mem.free".to_string(), stdlib_sysinfo_mem_free_stack);
    let _ = bc.vm.register_inline("sysinfo.mem.free.".to_string(), stdlib_sysinfo_mem_free_workbench);
    let _ = bc.vm.register_inline("sysinfo.mem.used".to_string(), stdlib_sysinfo_mem_used_stack);
    let _ = bc.vm.register_inline("sysinfo.mem.used.".to_string(), stdlib_sysinfo_mem_used_workbench);
    let _ = bc.vm.register_inline("sysinfo.mem.shared".to_string(), stdlib_sysinfo_mem_shared_stack);
    let _ = bc.vm.register_inline("sysinfo.mem.shared.".to_string(), stdlib_sysinfo_mem_shared_workbench);
    let _ = bc.vm.register_inline("sysinfo.mem.buffers".to_string(), stdlib_sysinfo_mem_buffers_stack);
    let _ = bc.vm.register_inline("sysinfo.mem.buffers.".to_string(), stdlib_sysinfo_mem_buffers_workbench);
    let _ = bc.vm.register_inline("sysinfo.mem.cached".to_string(), stdlib_sysinfo_mem_cached_stack);
    let _ = bc.vm.register_inline("sysinfo.mem.cached.".to_string(), stdlib_sysinfo_mem_cached_workbench);
    drop(bc);
}
