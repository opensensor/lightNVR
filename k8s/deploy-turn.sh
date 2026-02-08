#!/bin/bash
# Deploy TURN server for WebRTC relay
# This enables WebRTC through NAT/WireGuard without port forwarding

set -e

echo "=== Deploying TURN server for WebRTC ==="

# 1. Deploy coturn
echo "1. Deploying coturn..."
kubectl apply -f k8s/coturn.yaml

# 2. Deploy TCP services configmap
echo "2. Configuring nginx-ingress TCP passthrough..."
kubectl apply -f k8s/tcp-services-configmap.yaml

# 3. Check if ingress-nginx-controller has tcp-services-configmap arg
echo "3. Checking nginx-ingress-controller configuration..."
if ! kubectl get deployment ingress-nginx-controller -n ingress-nginx -o yaml | grep -q "tcp-services-configmap"; then
    echo "   Adding --tcp-services-configmap argument to ingress-nginx-controller..."
    kubectl patch deployment ingress-nginx-controller -n ingress-nginx --type='json' -p='[
      {
        "op": "add",
        "path": "/spec/template/spec/containers/0/args/-",
        "value": "--tcp-services-configmap=ingress-nginx/tcp-services"
      }
    ]'
else
    echo "   tcp-services-configmap already configured"
fi

# 4. Check if port 3478 is exposed on ingress-nginx service
echo "4. Exposing port 3478 on ingress-nginx service..."
if ! kubectl get svc ingress-nginx-controller -n ingress-nginx -o yaml | grep -q "3478"; then
    kubectl patch svc ingress-nginx-controller -n ingress-nginx --type='json' -p='[
      {
        "op": "add",
        "path": "/spec/ports/-",
        "value": {
          "name": "turn-tcp",
          "port": 3478,
          "targetPort": 3478,
          "protocol": "TCP"
        }
      }
    ]'
    echo "   Port 3478 added"
else
    echo "   Port 3478 already exposed"
fi

# 5. Update go2rtc config
echo "5. Updating go2rtc configuration..."
kubectl apply -f k8s/go2rtc-configmap.yaml

# 6. Restart lightnvr to pick up new config
echo "6. Restarting lightnvr deployment..."
kubectl rollout restart deployment/lightnvr -n lightnvr

# 7. Wait for rollout
echo "7. Waiting for rollout to complete..."
kubectl rollout status deployment/coturn -n lightnvr --timeout=60s
kubectl rollout status deployment/lightnvr -n lightnvr --timeout=120s

echo ""
echo "=== TURN server deployed ==="
echo ""
echo "TURN server: turn:nvr.opensensor.io:3478?transport=tcp"
echo "Username: lightnvr"
echo "Password: lightnvr-turn-2024"
echo ""
echo "WebRTC should now work through the TURN relay!"

