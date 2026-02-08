#!/bin/bash
set -e

echo "=== Setting up TURN server ==="

# 1. Apply coturn and configmaps
kubectl apply -f k8s/coturn.yaml
kubectl apply -f k8s/tcp-services-configmap.yaml

# 2. Add udp-services-configmap arg if missing
if ! kubectl get deployment ingress-nginx-controller -n ingress-nginx -o yaml | grep -q "udp-services-configmap"; then
    echo "Adding --udp-services-configmap to ingress-nginx-controller..."
    kubectl patch deployment ingress-nginx-controller -n ingress-nginx --type='json' -p='[
      {"op": "add", "path": "/spec/template/spec/containers/0/args/-", "value": "--udp-services-configmap=ingress-nginx/udp-services"}
    ]'
fi

# 3. Expose UDP ports on ingress-nginx service
echo "Patching ingress-nginx service with UDP ports..."
kubectl patch svc ingress-nginx-controller -n ingress-nginx --type='merge' -p='
spec:
  ports:
  - name: http
    port: 80
    protocol: TCP
    targetPort: 80
  - name: https
    port: 443
    protocol: TCP
    targetPort: 443
  - name: turn-tcp
    port: 3478
    protocol: TCP
    targetPort: 3478
  - name: turn-udp
    port: 3478
    protocol: UDP
    targetPort: 3478
  - name: relay-49152
    port: 49152
    protocol: UDP
    targetPort: 49152
  - name: relay-49153
    port: 49153
    protocol: UDP
    targetPort: 49153
  - name: relay-49154
    port: 49154
    protocol: UDP
    targetPort: 49154
  - name: relay-49155
    port: 49155
    protocol: UDP
    targetPort: 49155
  - name: relay-49156
    port: 49156
    protocol: UDP
    targetPort: 49156
  - name: relay-49157
    port: 49157
    protocol: UDP
    targetPort: 49157
  - name: relay-49158
    port: 49158
    protocol: UDP
    targetPort: 49158
  - name: relay-49159
    port: 49159
    protocol: UDP
    targetPort: 49159
  - name: relay-49160
    port: 49160
    protocol: UDP
    targetPort: 49160
  - name: relay-49161
    port: 49161
    protocol: UDP
    targetPort: 49161
  - name: relay-49162
    port: 49162
    protocol: UDP
    targetPort: 49162
  - name: relay-49163
    port: 49163
    protocol: UDP
    targetPort: 49163
  - name: relay-49164
    port: 49164
    protocol: UDP
    targetPort: 49164
  - name: relay-49165
    port: 49165
    protocol: UDP
    targetPort: 49165
  - name: relay-49166
    port: 49166
    protocol: UDP
    targetPort: 49166
  - name: relay-49167
    port: 49167
    protocol: UDP
    targetPort: 49167
  - name: relay-49168
    port: 49168
    protocol: UDP
    targetPort: 49168
  - name: relay-49169
    port: 49169
    protocol: UDP
    targetPort: 49169
  - name: relay-49170
    port: 49170
    protocol: UDP
    targetPort: 49170
  - name: relay-49171
    port: 49171
    protocol: UDP
    targetPort: 49171
  - name: relay-49172
    port: 49172
    protocol: UDP
    targetPort: 49172
'

# 4. Restart
kubectl rollout restart deployment/coturn -n lightnvr
kubectl rollout restart deployment/ingress-nginx-controller -n ingress-nginx

echo ""
echo "=== Done ==="
echo "Now point turn.opensensor.io DNS to 174.138.108.76"

