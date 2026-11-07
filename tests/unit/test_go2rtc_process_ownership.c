/* Real /proc socket ownership: do not substitute a hand-written column map. */
#include "../../src/video/go2rtc/go2rtc_process.c"
#include <arpa/inet.h>
#include "unity.h"

static int listener = -1;
void setUp(void) { listener = -1; }
void tearDown(void) { if (listener >= 0) close(listener); }

static void assert_listener_owner(int family) {
    listener = socket(family, SOCK_STREAM, 0);
    if (family == AF_INET6 && listener < 0) TEST_IGNORE_MESSAGE("IPv6 unavailable");
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, listener);
    int port;
    if (family == AF_INET) {
        struct sockaddr_in address = {.sin_family = AF_INET, .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
        TEST_ASSERT_EQUAL_INT(0, bind(listener, (struct sockaddr *)&address, sizeof(address)));
        socklen_t size = sizeof(address);
        TEST_ASSERT_EQUAL_INT(0, getsockname(listener, (struct sockaddr *)&address, &size));
        port = ntohs(address.sin_port);
    } else {
        struct sockaddr_in6 address = {.sin6_family = AF_INET6, .sin6_addr = IN6ADDR_LOOPBACK_INIT};
        TEST_ASSERT_EQUAL_INT(0, bind(listener, (struct sockaddr *)&address, sizeof(address)));
        socklen_t size = sizeof(address);
        TEST_ASSERT_EQUAL_INT(0, getsockname(listener, (struct sockaddr *)&address, &size));
        port = ntohs(address.sin6_port);
    }
    TEST_ASSERT_EQUAL_INT(0, listen(listener, 1));
    TEST_ASSERT_TRUE(process_owns_tcp_listen_port(getpid(), port));
    TEST_ASSERT_FALSE(process_owns_tcp_listen_port(getppid(), port));
    close(listener);
    listener = -1;
    TEST_ASSERT_FALSE(process_owns_tcp_listen_port(getpid(), port));
}
void test_ipv4_listener_inode_identifies_only_its_owner(void) { assert_listener_owner(AF_INET); }
void test_ipv6_listener_inode_identifies_only_its_owner(void) { assert_listener_owner(AF_INET6); }
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_ipv4_listener_inode_identifies_only_its_owner);
    RUN_TEST(test_ipv6_listener_inode_identifies_only_its_owner);
    return UNITY_END();
}
