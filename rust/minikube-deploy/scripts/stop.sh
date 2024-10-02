#!/bin/bash

MINIKUBE=$(which minikube)
KUBECTL="$MINIKUBE kubectl"
if [ -z "$MINIKUBE" ]; then
  echo "Minikube is not installed. Please install it first."
  echo '$ curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64'
  echo '$ sudo install minikube-linux-amd64 /usr/local/bin/minikube && rm minikube-linux-amd64'

  exit 1
fi

# delete all and stop
$KUBECTL delete pods --all

# stop
minikube stop
