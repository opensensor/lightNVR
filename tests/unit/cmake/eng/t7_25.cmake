add_layer2_test(test_t7_25)
target_link_options(test_t7_25 PRIVATE -Wl,--wrap=lstat -Wl,--wrap=fstatat
    -Wl,--wrap=openat -Wl,--wrap=close -Wl,--wrap=fstat -Wl,--wrap=mkdirat
    -Wl,--wrap=syscall -Wl,--wrap=unlinkat
    -Wl,--wrap=readdir -Wl,--wrap=lightnvr_uuid_generate_v4)
set_tests_properties(test_t7_25 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
