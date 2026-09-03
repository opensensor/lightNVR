add_layer2_test(test_system_health_thermal)
target_compile_definitions(test_system_health_thermal PRIVATE
    TEST_THERMAL_FIXTURE_DIR="${CMAKE_SOURCE_DIR}/tests/fixtures/system_health/sysfs/thermal"
)

add_layer2_test(test_system_health_network)
target_compile_definitions(test_system_health_network PRIVATE
    TEST_NETWORK_FIXTURE_DIR="${CMAKE_SOURCE_DIR}/tests/fixtures/system_health/sysfs/net"
)
