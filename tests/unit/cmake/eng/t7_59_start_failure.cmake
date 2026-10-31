add_layer3_test(test_t7_59_start_failure)
target_link_options(test_t7_59_start_failure PRIVATE
    -Wl,--wrap=pthread_mutex_lock -Wl,--wrap=pthread_create -Wl,--wrap=pthread_join)
set_tests_properties(test_t7_59_start_failure PROPERTIES TIMEOUT 30 LABELS "engineering;security")
