add_layer3_test(test_t7_59)
target_link_options(test_t7_59 PRIVATE
    -Wl,--wrap=clock_gettime -Wl,--wrap=pthread_create
    -Wl,--wrap=pthread_join -Wl,--wrap=pthread_cond_timedwait)
set_tests_properties(test_t7_59 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
