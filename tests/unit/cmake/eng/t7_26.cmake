add_layer2_test(test_t7_26)
target_link_options(test_t7_26 PRIVATE -Wl,--wrap=lstat -Wl,--wrap=syscall -Wl,--wrap=unlink)
set_tests_properties(test_t7_26 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
