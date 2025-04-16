# Databelt: A Continuous Data Path for Serverless Workflows in the 3D Compute Continuum

Databelt is a stateful serverless framework for the Edge-Cloud-Space 3D Compute Continuum. It introduces a Service Level Objective (SLO)-aware function state propagation mechanism and a function state fusion strategy to reduce latency and increase workflow efficiency in dynamic, satellite-powered environments.

## ✨ Key Features

- **SLO-Aware State Propagation**: Proactively migrates function states to optimal nodes based on network latency, node availability, and execution constraints.
- **Function State Fusion**: Co-locates related states within a shared sandbox to reduce redundant network and storage operations.
- **Optimized Scheduling**: Incorporates a distributed, pluggable scheduler that considers satellite availability, power, compute limits, and temperature.
- **Edge + Space Ready**: Lightweight implementation in Rust using WasmEdge, designed for constrained LEO satellites and edge nodes.

## 📊 Performance Highlights

- 🚀 Up to **66% reduction** in end-to-end workflow latency.
- ⚡ Up to **50% higher throughput** compared to stateless cloud-native models.
- 🌐 **79% local state availability**, reducing inter-node hops and latency.
- ✅ **0% SLO violations** under tested dynamic conditions.

## 🧠 Architecture Overview

Databelt has a 3-Phase Architecture: Identify, Compute, and Offload phases ensure efficient data placement across space, edge, and cloud nodes.

Its main components include:

- **Databelt Service**: Central service managing topology, SLOs, and state placement decisions.
- **Databelt Middleware**: Middleware library embedded in functions for transparent state access and propagation.
- **Databelt Scheduler**: Distributed scheduler with network-aware heuristics and resource filters, compatible with Kubernetes/Knative.



