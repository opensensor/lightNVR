add_layer2_test(test_smartctl_provider)
target_compile_definitions(test_smartctl_provider PRIVATE
    TEST_SMARTCTL_FIXTURE_DIR="${CMAKE_SOURCE_DIR}/tests/fixtures/system_health/smartctl"
)
