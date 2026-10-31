add_layer3_test(test_system_health_startup CI_UNIT)
target_link_options(test_system_health_startup PRIVATE
    -Wl,--wrap=pthread_cond_wait -Wl,--wrap=pthread_mutex_lock -Wl,--wrap=pthread_create -Wl,--wrap=pthread_join)
set_tests_properties(test_system_health_startup PROPERTIES TIMEOUT 30)
