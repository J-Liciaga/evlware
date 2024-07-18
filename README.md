# EVLWARE

EVLWARE is an advanced AI-driven penetration testing framework designed to revolutionize cybersecurity assessments. Leveraging cutting-edge machine learning techniques and a modern tech stack, EVLWARE automates and enhances various aspects of security testing, including vulnerability scanning, exploit generation, and social engineering simulations. Built on a scalable Nx monorepo architecture, it combines powerful AI models with robust security tools to provide comprehensive, adaptive, and intelligent security analysis. EVLWARE aims to stay ahead of emerging threats by continuously learning and evolving its testing methodologies, offering security professionals a state-of-the-art platform for identifying and addressing potential vulnerabilities in complex digital environments.

## Directory Structure

```
evlware/
├── .github/                 # Github Actions CI/CD files
├── .docker/                 # Dockerfiles files
├── .husky/                  # Husky Git hooks
├── apps/
    ├── api/                 # FastAPI backend
    └── dashboard/           # Next.js frontend
├── libs/
    ├── lucky-assets/        # Common UI assets
    ├── lucky-ui/            # UI component library
    ├── toolshed/            # Common UI tools
    ├── core/                # Shared core functionality
    ├── ml-models/           # PyTorch/TensorFlow models
    ├── network-tools/       # Scapy, Wireshark integrations
    ├── security-tools/      # OWASP ZAP, Metasploit integrations
    └── reporting/           # Report generation and visualization
├── tools/                   # Custom scripts and utilities

```
