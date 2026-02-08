  
**Product Requirements Document**

LightNVR Provisioning Layer

*nvr.opensensor.io*

| Version | 1.0 |
| :---- | :---- |
| **Date** | February 8, 2026 |
| **Author** | Matt / OpenSensor Engineering LLC |
| **Status** | Draft |
| **Classification** | Proprietary / Closed Source |

# **Table of Contents**

# **1\. Executive Summary**

The LightNVR Provisioning Layer is a closed-source, multi-tenant platform that enables users to self-provision and manage LightNVR instances hosted on DigitalOcean Kubernetes infrastructure. It transforms nvr.opensensor.io from a single demonstration instance into a fully automated provisioning and lifecycle management system for cloud-hosted video surveillance.

The platform integrates with the existing OpenSensor ecosystem: Fief for authentication and user management, Demetered (demetered.com) for usage metering and billing, coturn (turn.opensensor.io) for WebRTC relay, and WireGuard sidecar containers for secure connectivity to customer private networks. Users provision instances via a web dashboard, select resource tiers, configure network connectivity, and are billed prepaid with a 20% margin above operating costs.

This product is the first commercial SaaS offering built on top of the open-source LightNVR project and represents a new revenue stream for OpenSensor Engineering LLC.

# **2\. Problem Statement**

Self-hosting NVR software requires significant technical expertise: server provisioning, network configuration, TURN/STUN setup for WebRTC, storage management, SSL certificates, and ongoing maintenance. Most users who would benefit from lightweight, WebRTC-native video surveillance lack the infrastructure knowledge to deploy and operate it reliably.

Additionally, organizations with cameras on private networks (behind firewalls, on VLANs, in air-gapped environments) face additional complexity establishing secure connectivity between their camera networks and cloud-hosted NVR instances.

There is no existing managed NVR service built on open-source foundations that offers per-tenant isolation, automated network tunneling, and transparent usage-based pricing with prepaid billing.

# **3\. Objectives**

1. Enable non-technical users to provision fully operational LightNVR instances in under 5 minutes through a self-service web dashboard  
2. Automate WireGuard tunnel provisioning for secure connectivity to customer private camera networks  
3. Integrate tightly with Demetered for real-time cost tracking, prepaid billing, and margin enforcement at 20% above operating costs  
4. Provide per-tenant resource isolation with configurable CPU, memory, and storage allocations on DigitalOcean Kubernetes  
5. Maintain profitability from day one through prepaid billing and automated resource lifecycle management  
6. Lay groundwork for detections.opensensor.io (light-object-detect) as a companion service

# **4\. Target Users**

## **4.1 Primary: Small Business / Prosumer**

Users with 1–16 IP cameras who want cloud-hosted NVR without managing infrastructure. Typical scenarios include small retail, home security enthusiasts, and small office deployments. They value simplicity, predictable pricing, and WebRTC-based live viewing from any browser.

## **4.2 Secondary: IoT / Embedded Developers**

Developers building on Thingino or similar open-source camera firmware who need a managed NVR backend for testing, demos, or small production deployments. They appreciate the open-source LightNVR core and want managed infrastructure around it.

## **4.3 Tertiary: MSPs and Integrators**

Managed service providers who want to offer NVR-as-a-service to their own clients. They need multi-instance management, API-driven provisioning, and white-label potential.

# **5\. System Architecture**

## **5.1 Architecture Overview**

The provisioning layer operates as a control plane on top of the existing OpenSensor Kubernetes cluster in DigitalOcean. Each tenant instance is a namespaced deployment consisting of a LightNVR container, an optional WireGuard sidecar, and a persistent volume for recordings. The control plane manages the lifecycle of these deployments through the Kubernetes API.

| Component | Domain | Role |
| :---- | :---- | :---- |
| Provisioning Layer | nvr.opensensor.io | Control plane: auth, dashboard, instance lifecycle, billing integration |
| Fief Auth | members.opensensor.io | User authentication, registration, OAuth/OIDC, user management |
| Demetered | api.demetered.com | Usage metering, cost tracking, prepaid balance management, billing |
| TURN Server | turn.opensensor.io | coturn relay for WebRTC NAT traversal across all tenant instances |
| Detection Service | detections.opensensor.io | light-object-detect inference service (future phase) |
| Tenant Instances | nvr.opensensor.io/\<name\> | Individual LightNVR deployments with per-tenant isolation |

## **5.2 Tenant Instance Composition**

Each provisioned LightNVR instance is deployed as a Kubernetes pod with the following containers and resources:

| Container | Purpose | Configuration |
| :---- | :---- | :---- |
| lightnvr | Core NVR: stream ingestion, recording, WebRTC, HLS, go2rtc | CPU/memory per tier; mounts PVC for recordings |
| wireguard-sidecar | Encrypted tunnel to customer private network (optional) | Auto-generated keys; customer provides peer config |
| metrics-agent | Resource usage reporting to Demetered | Lightweight; reports CPU, memory, storage, bandwidth |

## **5.3 Network Architecture**

All tenant instances share the coturn service at turn.opensensor.io for WebRTC relay. Each instance is exposed via the NGINX ingress controller at a unique path (nvr.opensensor.io/\<instance-name\>) with SSL termination handled by cert-manager and Let’s Encrypt. WireGuard tunnels, when enabled, create point-to-point encrypted connections between the sidecar container and the customer’s network, allowing the LightNVR container to reach RTSP cameras on private subnets.

### **5.3.1 Network Allow-Listing**

Optionally, tenants can restrict access to their NVR instance by IP allowlist. This is enforced at the NGINX ingress level via annotations. The default policy is open access (authenticated via Fief), with allowlisting available as an opt-in security hardening measure.

## **5.4 Authentication Architecture**

The provisioning layer authenticates users through the existing Fief deployment at members.opensensor.io. Fief provides OAuth2/OIDC flows, user registration, and session management. The provisioning layer stores a mapping between Fief user IDs and tenant resources.

**Known Risk:** Fief is no longer actively maintained upstream. OpenSensor maintains a fork of all Fief components (server, Python client, JavaScript client). This is a long-term maintenance liability. The architecture should be designed with an authentication abstraction layer so that Fief can be replaced with an alternative (e.g., Authentik, Keycloak, or a custom solution) without rewriting the provisioning layer.

## **5.5 Billing Architecture**

The provisioning layer integrates with Demetered for all cost tracking and billing. The billing model is prepaid: users must have sufficient balance before provisioning or renewing resources. A 20% margin is applied above actual DigitalOcean operating costs.

### **5.5.1 Metering Events**

The metrics-agent sidecar in each tenant pod reports the following meters to Demetered:

* compute\_hours — CPU-hours consumed (based on tier allocation, not actual usage)  
* memory\_hours — GiB-hours allocated  
* storage\_gb — Persistent volume capacity provisioned  
* bandwidth\_gb — Egress bandwidth for WebRTC/HLS streaming  
* wireguard\_tunnels — Active WireGuard tunnel count  
* detection\_inferences — Object detection API calls (future, when detections.opensensor.io is active)

### **5.5.2 Prepaid Billing Flow**

1. User selects a resource tier and billing period (monthly or annual)  
2. Provisioning layer calculates cost: (DigitalOcean base cost × 1.20) for the selected period  
3. User pays via Stripe (or adds to prepaid balance)  
4. Demetered records the prepaid credit and begins tracking consumption against it  
5. At 80% consumption or 7 days before period end, user receives renewal notification  
6. If balance expires without renewal, instance is suspended (data retained for 30 days), then deleted

# **6\. Resource Tiers**

Users select a resource tier at provisioning time. Tiers can be changed (scaled up or down) with prorated billing. All prices shown are after the 20% margin.

| Tier | CPU | Memory | Storage | Streams | WG | Est. Monthly |
| :---- | :---- | :---- | :---- | :---- | :---- | :---- |
| Starter | 0.5 vCPU | 1 GiB | 20 GB | Up to 4 | 1 | $12/mo |
| Standard | 1 vCPU | 2 GiB | 50 GB | Up to 8 | 2 | $28/mo |
| Professional | 2 vCPU | 4 GiB | 100 GB | Up to 16 | 4 | $55/mo |
| Custom | Custom | Custom | Custom | Custom | Custom | Quoted |

Storage is provisioned as DigitalOcean Block Storage volumes. Additional storage can be added in 10 GB increments at $1.20/10 GB/month (DO cost $1.00 \+ 20% margin). Bandwidth overage beyond the base tier allocation is billed at $0.012/GB (DO cost $0.01 \+ 20%).

# **7\. Provisioning Workflow**

## **7.1 Instance Provisioning**

* User authenticates via Fief at nvr.opensensor.io  
* Dashboard shows existing instances and option to create new  
* User selects: instance name (becomes URL slug), resource tier, billing period, optional WireGuard tunnel configuration  
* System validates name availability and calculates prepaid cost  
* User completes payment (Stripe checkout or deducts from prepaid balance)  
* Control plane creates: Kubernetes namespace, LightNVR deployment, PVC, ingress rule, WireGuard sidecar (if configured), Demetered account and meters  
* Instance becomes available at nvr.opensensor.io/\<instance-name\> within 2–5 minutes

## **7.2 WireGuard Tunnel Provisioning**

When a user enables WireGuard connectivity:

* Control plane generates a WireGuard keypair for the sidecar  
* Dashboard displays the sidecar’s public key and endpoint for the user to add as a peer on their network’s WireGuard gateway  
* User provides: their WireGuard public key, their endpoint (IP:port), the allowed IPs (camera subnet CIDRs)  
* Control plane injects the WireGuard configuration into the sidecar container as a Kubernetes Secret and restarts the sidecar  
* The LightNVR container can now reach RTSP cameras on the customer’s private network via the sidecar’s tunnel interface  
* Dashboard shows tunnel status (connected/disconnected, last handshake, transfer stats)

## **7.3 Instance Lifecycle**

| State | Description | Transition |
| :---- | :---- | :---- |
| Provisioning | Resources being created in K8s | Automatic → Active on success |
| Active | Running and accessible | User action or billing → any state |
| Suspended | Stopped; data retained; billing paused | Renewal payment → Active; 30 days → Terminated |
| Scaling | Resource tier change in progress | Automatic → Active on completion |
| Terminated | Resources and data permanently deleted | Terminal state; name released after 90 days |

# **8\. API Specification**

The provisioning layer exposes a REST API at nvr.opensensor.io/api/v1/. All endpoints require Fief authentication via Bearer token. The API is the backend for the web dashboard and is also available for programmatic access by MSPs and integrators.

## **8.1 Core Endpoints**

| Method | Endpoint | Description |
| :---- | :---- | :---- |
| POST | /api/v1/instances | Provision a new LightNVR instance |
| GET | /api/v1/instances | List all instances for authenticated user |
| GET | /api/v1/instances/:id | Get instance details, status, and metrics |
| PATCH | /api/v1/instances/:id | Update instance (scale tier, rename, toggle features) |
| DELETE | /api/v1/instances/:id | Terminate instance (requires confirmation) |
| POST | /api/v1/instances/:id/suspend | Suspend instance (admin or billing-triggered) |
| POST | /api/v1/instances/:id/resume | Resume suspended instance (requires valid billing) |

## **8.2 WireGuard Endpoints**

| Method | Endpoint | Description |
| :---- | :---- | :---- |
| GET | /api/v1/instances/:id/tunnels | List WireGuard tunnels for instance |
| POST | /api/v1/instances/:id/tunnels | Create new WireGuard tunnel (returns server pubkey) |
| PATCH | /api/v1/instances/:id/tunnels/:tid | Update tunnel peer config |
| DELETE | /api/v1/instances/:id/tunnels/:tid | Remove WireGuard tunnel |
| GET | /api/v1/instances/:id/tunnels/:tid/status | Tunnel health: handshake, transfer, latency |

## **8.3 Billing Endpoints**

| Method | Endpoint | Description |
| :---- | :---- | :---- |
| GET | /api/v1/billing/balance | Current prepaid balance and usage summary |
| POST | /api/v1/billing/checkout | Create Stripe checkout session for prepaid credit |
| GET | /api/v1/billing/invoices | Historical invoices and payment records |
| GET | /api/v1/billing/usage | Detailed usage breakdown from Demetered |
| POST | /api/v1/billing/estimate | Estimate cost for a given tier and period |

## **8.4 Network Allow-List Endpoints**

| Method | Endpoint | Description |
| :---- | :---- | :---- |
| GET | /api/v1/instances/:id/allowlist | Get current IP allowlist |
| PUT | /api/v1/instances/:id/allowlist | Replace allowlist (array of CIDRs) |
| DELETE | /api/v1/instances/:id/allowlist | Remove allowlist (open access) |

# **9\. Data Model**

The provisioning layer uses PostgreSQL for its control plane database. This is separate from the individual LightNVR instance databases (SQLite within each container).

## **9.1 Core Entities**

| Entity | Key Fields | Notes |
| :---- | :---- | :---- |
| Tenant | id, fief\_user\_id, email, created\_at, status | Maps 1:1 to Fief user; stores billing preferences |
| Instance | id, tenant\_id, name, slug, tier, state, k8s\_namespace | Core resource; slug becomes URL path component |
| WireGuardTunnel | id, instance\_id, server\_pubkey, peer\_pubkey, peer\_endpoint, allowed\_ips | Secrets stored encrypted; config injected as K8s Secret |
| BillingPeriod | id, tenant\_id, instance\_id, start, end, prepaid\_amount, consumed | Tracks prepaid balance per instance per period |
| AllowlistEntry | id, instance\_id, cidr, description | Optional IP restrictions on ingress |
| AuditLog | id, tenant\_id, action, resource\_type, resource\_id, timestamp | Immutable log of all provisioning actions |

# **10\. Detection Service Integration (Future Phase)**

The detections.opensensor.io service will run light-object-detect as a shared inference endpoint. LightNVR instances will be configurable to send detection requests to this service, with usage metered through Demetered.

## **10.1 Architecture**

* Deployed as a separate service in the same Kubernetes cluster  
* Shared GPU node pool (if available) or CPU-based inference  
* LightNVR instances send frames via internal cluster networking (no public egress)  
* Each inference call is metered as a detection\_inferences event in Demetered  
* Detection results returned to the LightNVR instance for recording triggers, alerting, and metadata tagging

## **10.2 Pricing Model**

Detection will be billed per-inference with tiered pricing. Tenants opt in per-instance and can set inference frequency (e.g., every Nth frame, on motion trigger only). Estimated pricing: $0.001 per inference with volume discounts.

# **11\. Security Considerations**

* Tenant isolation: Each instance runs in a dedicated Kubernetes namespace with NetworkPolicy restricting cross-namespace traffic  
* WireGuard keys: Private keys stored as Kubernetes Secrets encrypted at rest (DigitalOcean managed encryption); never exposed via API  
* RTSP credentials: Camera credentials stored encrypted in the LightNVR instance database; never transmitted to the control plane  
* SSL/TLS: All public endpoints terminated with Let’s Encrypt via cert-manager; internal traffic over cluster networking  
* Audit logging: All provisioning actions logged immutably with tenant ID, action type, and timestamp  
* CORS: Restricted to \*.opensensor.io origins (consistent with existing Fief CORS policy)  
* Authentication abstraction: Fief integration behind an interface to enable future migration to an alternative identity provider

# **12\. Known Risks and Mitigations**

| Risk | Severity | Mitigation |
| :---- | :---- | :---- |
| Fief maintenance burden (unmaintained upstream) | High | Abstract auth behind interface; maintain fork minimally; evaluate Authentik/Keycloak migration on 6-month cadence |
| Noisy neighbor (shared cluster) | Medium | Kubernetes ResourceQuotas and LimitRanges per namespace; monitor via Demetered metrics |
| WireGuard config complexity for end users | Medium | Provide downloadable config files, QR codes for mobile, and step-by-step setup wizard in dashboard |
| DigitalOcean cost volatility | Low | Prepaid billing insulates from short-term cost changes; review margin quarterly |
| Data loss on termination | Medium | 30-day grace period on suspension; optional backup to DO Spaces before termination; clear warnings in UI |
| coturn capacity under load | Medium | Monitor TURN relay bandwidth; scale coturn horizontally; per-tenant TURN credential isolation |

# **13\. Development Phases**

## **Phase 1: Foundation (Weeks 1–4)**

* Control plane API (FastAPI or Go) with PostgreSQL  
* Fief authentication integration with abstraction layer  
* Kubernetes operator or controller for instance lifecycle (create, delete, suspend)  
* Basic web dashboard: login, instance list, create instance  
* Ingress routing for tenant instances (nvr.opensensor.io/\<name\>)  
* Integration with existing coturn at turn.opensensor.io

## **Phase 2: Billing and Networking (Weeks 5–8)**

* Demetered integration: metrics-agent sidecar, usage tracking, balance queries  
* Stripe checkout integration for prepaid billing  
* Automated WireGuard sidecar provisioning and key management  
* Tunnel status monitoring and dashboard display  
* IP allowlist enforcement at ingress level  
* Suspension/termination lifecycle automation based on billing state

## **Phase 3: Polish and Launch (Weeks 9–12)**

* Dashboard UX refinement: setup wizard, resource monitoring graphs, billing history  
* Tier scaling (upgrade/downgrade) with prorated billing  
* Email notifications: provisioning complete, billing reminders, suspension warnings  
* Documentation: API reference, WireGuard setup guides, FAQ  
* Load testing and security audit  
* Beta launch with select users

## **Phase 4: Detection Service (Weeks 13–16)**

* Deploy light-object-detect at detections.opensensor.io  
* LightNVR integration: configuration UI for detection endpoints, frequency, and triggers  
* Demetered metering for detection\_inferences  
* Detection-triggered recording and alerting in tenant instances

# **14\. Success Metrics**

| Metric | Target (3 months) | Target (6 months) |
| :---- | :---- | :---- |
| Active tenant instances | 10 | 50 |
| Provisioning time (P95) | \< 5 minutes | \< 3 minutes |
| Instance uptime | 99.5% | 99.9% |
| Gross margin | ≥ 20% | ≥ 25% |
| Monthly recurring revenue | $300 | $1,500 |
| WireGuard tunnel adoption rate | 30% of instances | 50% of instances |
| Customer churn (monthly) | \< 10% | \< 5% |

# **15\. Open Questions**

* Fief long-term strategy: At what point does maintaining the fork become untenable, and what is the migration plan? Should we start with a Fief abstraction layer or defer until it becomes a problem?

* Custom domain support: Should tenants be able to CNAME their own domain to their instance? This adds SSL complexity (wildcard certs or per-domain cert provisioning).

* Multi-region: Should we plan for DigitalOcean regions beyond the initial cluster, or keep single-region until demand justifies it?

* Recording backup/export: Should we offer automated backup to DigitalOcean Spaces or user-provided S3-compatible storage?

* Control plane language: FastAPI (Python, consistent with Demetered and existing tooling) vs Go (better fit for Kubernetes operator pattern)?

* MSP/white-label: How much effort should Phase 1 invest in multi-tenant admin capabilities for MSP use cases?

*— End of Document —*