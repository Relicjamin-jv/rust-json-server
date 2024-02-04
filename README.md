# Prerequisite

There are a few things that need to be installed before running this service on a kubernetes kluser locally.

#### Things that need to be installed:
- [K3s](https://k3s.io/)
- [Helm](https://helm.sh/docs/intro/install/)

# Getting Started

Build the container with either Podman or Buildah:
#### With Podman:
```
podman build -t rust-httpd .
```

#### With Buildah:
```
buildah build -t rust-httpd .
```

#### Import the Image to K3s:
Once that is completed, import the image to k3s local registry:
```
k3s ctr images import localhost/rust-httpd:latest
```

#### Run the Helm Chart
```
helm install rust-httpd ./helm
```

#### Test it 
Open a browser and access it at localhost:8000
