#include "video/onvif_discovery_messages.h"
#include <stdio.h>
#include <stdlib.h>

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
    snprintf(uuid, size, "%x%x-%x-%x-%x-%x%x%x",
             rand() & 0xffff, rand() & 0xffff,
             rand() & 0xffff,
             ((rand() & 0x0fff) | 0x4000),
             ((rand() & 0x3fff) | 0x8000),
             rand() & 0xffff, rand() & 0xffff, rand() & 0xffff);
}
