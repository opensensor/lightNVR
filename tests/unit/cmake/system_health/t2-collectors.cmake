add_layer2_test(test_system_health_proc)
target_compile_definitions(test_system_health_proc PRIVATE
    TEST_FIXTURE_ROOT="${CMAKE_SOURCE_DIR}/tests/fixtures/system_health"
)

add_layer2_test(test_system_health_cgroup)
target_compile_definitions(test_system_health_cgroup PRIVATE
    TEST_FIXTURE_ROOT="${CMAKE_SOURCE_DIR}/tests/fixtures/system_health"
)
