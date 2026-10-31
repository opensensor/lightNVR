add_layer2_test(test_t7_25)
target_link_options(test_t7_25 PRIVATE -Wl,--wrap=lstat -Wl,--wrap=fstatat)
set_tests_properties(test_t7_25 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
