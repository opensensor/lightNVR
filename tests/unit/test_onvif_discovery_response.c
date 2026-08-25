#include <string.h>

#include "unity.h"
#include "video/onvif_discovery_response.h"

void setUp(void) {}
void tearDown(void) {}

void test_endpoint_reference_is_stable_identity(void) {
    const char *response =
        "<s:Envelope><s:Body><d:ProbeMatches><d:ProbeMatch>"
        "<a:EndpointReference><a:Address>"
        "urn:uuid:01234567-89ab-cdef-0123-456789abcdef"
        "</a:Address></a:EndpointReference>"
        "<d:Types>dn:NetworkVideoTransmitter</d:Types>"
        "<d:XAddrs>http://192.0.2.70/onvif/device_service</d:XAddrs>"
        "</d:ProbeMatch></d:ProbeMatches></s:Body></s:Envelope>";
    onvif_device_info_t device;
    TEST_ASSERT_EQUAL_INT(0, parse_device_info(response, &device));
    TEST_ASSERT_EQUAL_STRING(
        "urn:uuid:01234567-89ab-cdef-0123-456789abcdef", device.endpoint);
    TEST_ASSERT_EQUAL_STRING(
        "http://192.0.2.70/onvif/device_service", device.device_service);
    TEST_ASSERT_EQUAL_STRING("192.0.2.70", device.ip_address);
}

void test_xaddr_is_identity_fallback_when_epr_is_missing(void) {
    const char *response =
        "<Envelope><Body><ProbeMatches><ProbeMatch>"
        "<Types>NetworkVideoTransmitter</Types>"
        "<XAddrs>http://camera.local/onvif/device_service</XAddrs>"
        "</ProbeMatch></ProbeMatches></Body></Envelope>";
    onvif_device_info_t device;
    TEST_ASSERT_EQUAL_INT(0, parse_device_info(response, &device));
    TEST_ASSERT_EQUAL_STRING(device.device_service, device.endpoint);
    TEST_ASSERT_EQUAL_STRING("camera.local", device.ip_address);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_endpoint_reference_is_stable_identity);
    RUN_TEST(test_xaddr_is_identity_fallback_when_epr_is_missing);
    return UNITY_END();
}
