"""Reviewed mechanical adaptations applied to raw C2Rust output.

Every adaptation is keyed by C source file and must still match the raw
output; a stale entry fails generation so this table cannot silently rot.
These fix compiler errors only. They are not a safety review.
"""
import re

# Copy Option<fn> values through raw pointers before calling Option methods,
# so no Rust shared reference to a mutable C global is created.
OPTION_FN_READS = {
    "src/database/db_core.c": ["scheduled_backup_cycle_override"],
    "src/telemetry/system_health_evaluator.c": ["service.transition_sink"],
    "src/video/sod_realnet.c": [
        "sod_realnet_funcs.sod_realnet_create",
        "sod_realnet_funcs.sod_realnet_load_model_from_mem",
        "sod_realnet_funcs.sod_realnet_model_config",
        "sod_realnet_funcs.sod_realnet_detect",
        "sod_realnet_funcs.sod_realnet_destroy",
    ],
}

# The stored value is T*, while the destination variable is const T*.
# Specify the intrinsic's stored type; leave its ordering intact.
TYPED_POINTER_INTRINSICS = {
    "src/video/hls/hls_unified_thread.c": [
        ("writer_ptr_0 = ::core::intrinsics::atomic_load", "*mut hls_writer_t"),
        ("writer_to_cleanup = ::core::intrinsics::atomic_xchg", "*mut hls_writer_t"),
    ],
}

# C atomic_bool locations rewritten to typed AtomicBool operations.
BOOLEAN_ATOMICS = {
    "src/web/go2rtc_proxy_thread.c": ["g_proxy_state.shutting_down"],
    "src/video/onvif_detection.c": ["warned_missing_key"],
    "src/video/unified_detection_thread.c": ["(*ctx).fps_is_provisional"],
    "src/video/recording_transcode.c": ["s_stopping"],
    "src/storage/storage_migration.c": ["stop_requested"],
    "src/database/db_backup.c": ["backup_cancel_requested"],
    "src/video/hls/hls_unified_thread.c": ["watchdog_running"],
    "src/telemetry/providers/builtin_providers.c": ["smartctl_active"],
    "src/web/recording_archive.c": ["(*stream).cancelled"],
    "src/telemetry/providers/smartctl_provider.c": ["(*state).refresh_pending"],
    "src/telemetry/recording_io_metrics.c": ["device_refresh_requested"],
    "src/core/shutdown_coordinator.c": [
        "g_background_abort_requested",
        "g_coordinator.shutdown_initiated",
        "g_coordinator.coordinator_destroyed",
    ],
    "src/storage/storage_source.c": ["source_stop"],
    "src/telemetry/system_health.c": ["(*slot).busy", "(*state).running"],
}


class StaleRepair(RuntimeError):
    pass


def _boolean_atomics(source, file, targets):
    counts = dict.fromkeys(targets, 0)
    locations = "|".join(re.escape(t) for t in targets)
    pattern = (r"::core::intrinsics::atomic_(load|store|xchg|cxchg)::<([^<>]*)>\(\s*&raw mut ("
               + locations + r")\s*,?\s*([^()]*)\)")

    def replace(match):
        operation, generics, target, args = match.groups()
        counts[target] += 1
        orders = ["::core::sync::atomic::Ordering::" + o
                  for o in re.findall(r"AtomicOrdering::(\w+)", generics)]
        values = [v.strip() for v in args.strip().rstrip(",").split(",") if v.strip()]
        base = f"::core::sync::atomic::AtomicBool::from_ptr(&raw mut {target})"
        if operation == "cxchg":
            if len(orders) != 2 or len(values) != 2:
                raise StaleRepair(f"{file}: unexpected cxchg form: {match.group(0)}")
            call = base + ".compare_exchange(" + ", ".join(values + orders) + ")"
            # Intrinsics return (observed old value, success), even on failure.
            return f"match {call} {{ Ok(old) => (old, true), Err(old) => (old, false) }}"
        if len(orders) != 1 or len(values) != (operation != "load"):
            raise StaleRepair(f"{file}: unexpected {operation} form: {match.group(0)}")
        method = {"load": "load", "store": "store", "xchg": "swap"}[operation]
        return base + "." + method + "(" + ", ".join(values + orders) + ")"

    result = re.sub(pattern, replace, source, flags=re.S)
    missing = [t for t, n in counts.items() if n == 0]
    if missing:
        raise StaleRepair(f"{file}: atomic_bool repair targets no longer present: {missing}")
    return result


def apply(file, source):
    """Return adapted Rust for C `file` (repo-relative)."""
    for expr in OPTION_FN_READS.get(file, []):
        needle = expr + ".is_some()"
        if needle not in source:
            raise StaleRepair(f"{file}: Option<fn> repair target missing: {needle}")
        source = source.replace(needle, f"(&raw const {expr}).read().is_some()")
    for prefix, stored in TYPED_POINTER_INTRINSICS.get(file, []):
        source, n = re.subn(re.escape(prefix) + r"::<\s*_,", prefix + f"::<\n            {stored},", source)
        if n != 1:
            raise StaleRepair(f"{file}: typed intrinsic repair matched {n} times: {prefix}")
    if file in BOOLEAN_ATOMICS:
        source = _boolean_atomics(source, file, BOOLEAN_ATOMICS[file])
    return source
