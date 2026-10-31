add_layer2_test(test_t7_27)
target_link_options(test_t7_27 PRIVATE -Wl,--wrap=fstat -Wl,--wrap=syscall -Wl,--wrap=mkdtemp -Wl,--wrap=access -Wl,--wrap=rename -Wl,--wrap=db_operator_floor_plan_set_background)
set_tests_properties(test_t7_27 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
