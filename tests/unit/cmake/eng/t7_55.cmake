add_layer3_test(test_t7_55)
target_link_options(test_t7_55 PRIVATE
    -Wl,--wrap=fopen -Wl,--wrap=fgets -Wl,--wrap=fclose)
set_tests_properties(test_t7_55 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
