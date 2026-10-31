add_layer3_test(test_t7_25_retention)
target_link_options(test_t7_25_retention PRIVATE -Wl,--wrap=stat)
set_tests_properties(test_t7_25_retention PROPERTIES TIMEOUT 30 LABELS "engineering;security")
