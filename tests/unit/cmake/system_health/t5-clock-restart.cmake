add_layer2_test(test_system_health_clock)
add_layer2_test(test_system_health_restart)
target_compile_definitions(test_system_health_restart PRIVATE
    TEST_CLOCK_FIXTURE_DIR="${CMAKE_SOURCE_DIR}/tests/fixtures/system_health/clock"
)
