/**
 * @file test_onvif_soap_fault.c
 * @brief Layer 2 unit tests for onvif_log_soap_fault()
 *
 * Exercises the SOAP fault parsing logic in onvif_soap.c with various
 * XML payloads: SOAP 1.2 faults with code/subcode/reason, SOAP 1.1
 * faultstring, missing elements, unparseable XML, and edge cases.
 *
 * The function under test only logs — it has no return value — so these
 * tests verify that the function does not crash or leak on each input.
 * Coverage is the primary goal.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "unity.h"
#include "core/logger.h"
#include "video/onvif_soap.h"

void setUp(void) {}
void tearDown(void) {}

/* ================================================================
 * SOAP 1.2 fault with Code, Subcode, and Reason (the issue example)
 * ================================================================ */
void test_soap12_full_fault(void) {
    const char *xml =
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>"
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" "
        "xmlns:ter=\"http://www.onvif.org/ver10/error\">"
        "<s:Body><s:Fault>"
        "<s:Code><s:Value>s:Sender</s:Value>"
        "<s:Subcode><s:Value>ter:NotAuthorized</s:Value></s:Subcode></s:Code>"
        "<s:Reason><s:Text xml:lang=\"en\">Sender not Authorized</s:Text></s:Reason>"
        "</s:Fault></s:Body></s:Envelope>";

    onvif_log_soap_fault(xml, strlen(xml), "TestFull");
    /* No crash = pass */
    TEST_PASS();
}

/* ================================================================
 * SOAP 1.2 fault with Code but no Subcode
 * ================================================================ */
void test_soap12_code_no_subcode(void) {
    const char *xml =
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\">"
        "<s:Body><s:Fault>"
        "<s:Code><s:Value>s:Receiver</s:Value></s:Code>"
        "<s:Reason><s:Text>Internal error</s:Text></s:Reason>"
        "</s:Fault></s:Body></s:Envelope>";

    onvif_log_soap_fault(xml, strlen(xml), "TestNoSubcode");
    TEST_PASS();
}

/* ================================================================
 * SOAP 1.2 fault with Reason only (no Code element)
 * ================================================================ */
void test_soap12_reason_only(void) {
    const char *xml =
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\">"
        "<s:Body><s:Fault>"
        "<s:Reason><s:Text>Something went wrong</s:Text></s:Reason>"
        "</s:Fault></s:Body></s:Envelope>";

    onvif_log_soap_fault(xml, strlen(xml), "TestReasonOnly");
    TEST_PASS();
}

/* ================================================================
 * SOAP 1.1 style fault with faultstring
 * ================================================================ */
void test_soap11_faultstring(void) {
    const char *xml =
        "<SOAP-ENV:Envelope xmlns:SOAP-ENV=\"http://www.w3.org/2003/05/soap-envelope\">"
        "<SOAP-ENV:Body><SOAP-ENV:Fault>"
        "<faultstring>Authentication failed</faultstring>"
        "</SOAP-ENV:Fault></SOAP-ENV:Body></SOAP-ENV:Envelope>";

    onvif_log_soap_fault(xml, strlen(xml), "TestSOAP11");
    TEST_PASS();
}

/* ================================================================
 * Valid XML body but no Fault element
 * ================================================================ */
void test_no_fault_element(void) {
    const char *xml =
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\">"
        "<s:Body><s:SomeResponse>OK</s:SomeResponse></s:Body></s:Envelope>";

    onvif_log_soap_fault(xml, strlen(xml), "TestNoFault");
    TEST_PASS();
}

/* ================================================================
 * Unparseable / garbage XML
 * ================================================================ */
void test_unparseable_xml(void) {
    const char *garbage = "This is not XML at all <><><";
    onvif_log_soap_fault(garbage, strlen(garbage), "TestGarbage");
    TEST_PASS();
}

/* ================================================================
 * NULL and empty inputs
 * ================================================================ */
void test_null_response(void) {
    onvif_log_soap_fault(NULL, 0, "TestNull");
    TEST_PASS();
}

void test_empty_response(void) {
    char buf[1] = "";
    onvif_log_soap_fault(buf, 0, "TestEmpty");
    TEST_PASS();
}

void test_null_context(void) {
    const char *xml =
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\">"
        "<s:Body><s:Fault>"
        "<s:Code><s:Value>s:Sender</s:Value></s:Code>"
        "<s:Reason><s:Text>Error</s:Text></s:Reason>"
        "</s:Fault></s:Body></s:Envelope>";

    onvif_log_soap_fault(xml, strlen(xml), NULL);
    TEST_PASS();
}

/* ================================================================
 * Fault element present but completely empty (no Code, Reason, faultstring)
 * ================================================================ */
void test_empty_fault(void) {
    const char *xml =
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\">"
        "<s:Body><s:Fault></s:Fault></s:Body></s:Envelope>";

    onvif_log_soap_fault(xml, strlen(xml), "TestEmptyFault");
    TEST_PASS();
}

/* ================================================================
 * SOAP-ENV namespace prefix (common in some cameras)
 * ================================================================ */
void test_soap_env_namespace(void) {
    const char *xml =
        "<SOAP-ENV:Envelope xmlns:SOAP-ENV=\"http://www.w3.org/2003/05/soap-envelope\">"
        "<SOAP-ENV:Body><SOAP-ENV:Fault>"
        "<SOAP-ENV:Code><SOAP-ENV:Value>SOAP-ENV:Sender</SOAP-ENV:Value>"
        "<SOAP-ENV:Subcode><SOAP-ENV:Value>ter:ActionNotSupported</SOAP-ENV:Value>"
        "</SOAP-ENV:Subcode></SOAP-ENV:Code>"
        "<SOAP-ENV:Reason><SOAP-ENV:Text>Not supported</SOAP-ENV:Text></SOAP-ENV:Reason>"
        "</SOAP-ENV:Fault></SOAP-ENV:Body></SOAP-ENV:Envelope>";

    onvif_log_soap_fault(xml, strlen(xml), "TestSOAPENV");
    TEST_PASS();
}

/* ================================================================
 * Reolink-style fault: Code present but no Reason/Text element (#374)
 * Exercises the raw-XML-snippet diagnostic branch in onvif_log_soap_fault.
 * ================================================================ */
void test_fault_code_no_reason(void) {
    const char *xml =
        "<SOAP-ENV:Envelope xmlns:SOAP-ENV=\"http://www.w3.org/2003/05/soap-envelope\">"
        "<SOAP-ENV:Body><SOAP-ENV:Fault>"
        "<SOAP-ENV:Code><SOAP-ENV:Value>SOAP-ENV:Sender</SOAP-ENV:Value></SOAP-ENV:Code>"
        "</SOAP-ENV:Fault></SOAP-ENV:Body></SOAP-ENV:Envelope>";

    onvif_log_soap_fault(xml, strlen(xml), "TestReolink");
    TEST_PASS();
}

/* ================================================================
 * Uppercase S namespace prefix
 * ================================================================ */
void test_uppercase_s_namespace(void) {
    const char *xml =
        "<S:Envelope xmlns:S=\"http://www.w3.org/2003/05/soap-envelope\">"
        "<S:Body><S:Fault>"
        "<S:Code><S:Value>S:Sender</S:Value></S:Code>"
        "<S:Reason><S:Text>Bad request</S:Text></S:Reason>"
        "</S:Fault></S:Body></S:Envelope>";

    onvif_log_soap_fault(xml, strlen(xml), "TestUpperS");
    TEST_PASS();
}

/* ================================================================
 * main
 * ================================================================ */
int main(void) {
    init_logger();

    UNITY_BEGIN();

    RUN_TEST(test_soap12_full_fault);
    RUN_TEST(test_soap12_code_no_subcode);
    RUN_TEST(test_soap12_reason_only);
    RUN_TEST(test_soap11_faultstring);
    RUN_TEST(test_no_fault_element);
    RUN_TEST(test_unparseable_xml);
    RUN_TEST(test_null_response);
    RUN_TEST(test_empty_response);
    RUN_TEST(test_null_context);
    RUN_TEST(test_empty_fault);
    RUN_TEST(test_soap_env_namespace);
    RUN_TEST(test_fault_code_no_reason);
    RUN_TEST(test_uppercase_s_namespace);

    return UNITY_END();
}


