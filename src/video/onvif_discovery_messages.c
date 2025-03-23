#include "video/onvif_discovery_messages.h"
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// ONVIF WS-Discovery message templates
// Standard ONVIF WS-Discovery message
const char *ONVIF_DISCOVERY_MSG = 
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
    "<e:Envelope xmlns:e=\"http://www.w3.org/2003/05/soap-envelope\" "
    "xmlns:w=\"http://schemas.xmlsoap.org/ws/2004/08/addressing\" "
    "xmlns:d=\"http://schemas.xmlsoap.org/ws/2005/04/discovery\" "
    "xmlns:dn=\"http://www.onvif.org/ver10/network/wsdl\">"
    "<e:Header>"
    "<w:MessageID>uuid:%s</w:MessageID>"
    "<w:To>urn:schemas-xmlsoap-org:ws:2005:04:discovery</w:To>"
    "<w:Action>http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</w:Action>"
    "</e:Header>"
    "<e:Body>"
    "<d:Probe>"
    "<d:Types>dn:NetworkVideoTransmitter</d:Types>"
    "</d:Probe>"
    "</e:Body>"
    "</e:Envelope>";

// Alternative ONVIF WS-Discovery message (some devices use different namespaces)
const char *ONVIF_DISCOVERY_MSG_ALT = 
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
    "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" "
    "xmlns:a=\"http://schemas.xmlsoap.org/ws/2004/08/addressing\" "
    "xmlns:d=\"http://schemas.xmlsoap.org/ws/2005/04/discovery\">"
    "<s:Header>"
    "<a:MessageID>uuid:%s</a:MessageID>"
    "<a:To>urn:schemas-xmlsoap-org:ws:2005:04:discovery</a:To>"
    "<a:Action>http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</a:Action>"
    "</s:Header>"
    "<s:Body>"
    "<d:Probe>"
    "<d:Types xmlns:tds=\"http://www.onvif.org/ver10/device/wsdl\">tds:Device</d:Types>"
    "</d:Probe>"
    "</s:Body>"
    "</s:Envelope>";

// WS-Discovery message with scope
const char *ONVIF_DISCOVERY_MSG_WITH_SCOPE = 
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
    "<e:Envelope xmlns:e=\"http://www.w3.org/2003/05/soap-envelope\" "
    "xmlns:w=\"http://schemas.xmlsoap.org/ws/2004/08/addressing\" "
    "xmlns:d=\"http://schemas.xmlsoap.org/ws/2005/04/discovery\" "
    "xmlns:dn=\"http://www.onvif.org/ver10/network/wsdl\">"
    "<e:Header>"
    "<w:MessageID>uuid:%s</w:MessageID>"
    "<w:To>urn:schemas-xmlsoap-org:ws:2005:04:discovery</w:To>"
    "<w:Action>http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</w:Action>"
    "</e:Header>"
    "<e:Body>"
    "<d:Probe>"
    "<d:Types>dn:NetworkVideoTransmitter</d:Types>"
    "<d:Scopes>onvif://www.onvif.org</d:Scopes>"
    "</d:Probe>"
    "</e:Body>"
    "</e:Envelope>";

// Generate a random UUID for WS-Discovery message ID
void generate_uuid(char *uuid, size_t size) {
    // Use a more deterministic approach for UUID generation
    // This ensures consistent UUIDs across different crypto libraries
    
    // Get current time for some randomness
    time_t now = time(NULL);
    unsigned int seed = (unsigned int)now;
    
    // Use a local random state to avoid affecting the global rand() state
    unsigned int r1 = seed ^ 0x12345678;
    unsigned int r2 = (seed >> 8) ^ 0x87654321;
    unsigned int r3 = (seed >> 16) ^ 0xabcdef01;
    unsigned int r4 = (seed >> 24) ^ 0x10fedcba;
    
    // Generate UUID components
    unsigned int p1 = (r1 * 1103515245 + 12345) & 0xffff;
    unsigned int p2 = (r2 * 1103515245 + 12345) & 0xffff;
    unsigned int p3 = (r3 * 1103515245 + 12345) & 0xffff;
    unsigned int p4 = (r4 * 1103515245 + 12345) & 0xffff;
    
    // Format according to UUID v4 format (random)
    snprintf(uuid, size, "%04x%04x-%04x-%04x-%04x-%04x%04x%04x",
             p1, p2,
             p3,
             ((p4 & 0x0fff) | 0x4000), // Version 4
             ((r1 & 0x3fff) | 0x8000), // Variant 1
             r2 & 0xffff, r3 & 0xffff, r4 & 0xffff);
}
