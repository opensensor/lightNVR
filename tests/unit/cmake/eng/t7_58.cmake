add_layer3_test(test_t7_58)
target_link_options(test_t7_58 PRIVATE -Wl,--wrap=fopen -Wl,--wrap=fclose)
set_tests_properties(test_t7_58 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
