add_layer2_test(test_system_health_hardware)
target_compile_definitions(test_system_health_hardware PRIVATE
    TEST_HARDWARE_FIXTURE_DIR="${CMAKE_SOURCE_DIR}/tests/fixtures/system_health/sysfs/hardware"
)

add_layer2_test(test_kernel_log_provider)
target_compile_definitions(test_kernel_log_provider PRIVATE
    TEST_KERNEL_LOG_FIXTURE_DIR="${CMAKE_SOURCE_DIR}/tests/fixtures/system_health/kernel_log"
)
