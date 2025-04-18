# Simple SHA256 Checker

## Overview

The **Simple SHA256 Checker** allows you to easily verify the integrity of files by comparing their SHA256 hash values against known trusted hashes. It is a straightforward solution for detecting obvious malicious code, **obviously not intended for advanced security analysis**.

---

## Features

- **Fast and Easy**: Quickly check if a file's hash matches a known good hash.
- **Recursive Checking**: The tool checks **EXE** and **DLL** files recursively to ensure that all relevant files in a directory are verified.
- **Malicious Code Detection**: Useful for identifying blatantly malicious files.
- **Known SHA256 Hashes**: Download and select a TXT list of known SHA256 hashes from [Bazaar Abuse](https://bazaar.abuse.ch/export/) to compare against.

---

## How It Works

1. **Select a Directory**: Choose the directory containing the **EXE** and **DLL** files you want to check.
2. **Download and Select Trusted Hash List**: Download a TXT list of known trusted SHA256 hashes from [Bazaar Abuse](https://bazaar.abuse.ch/export/), and upload it into the tool.
3. **Recursive Scan**: The tool will recursively scan all **EXE** and **DLL** files in the selected directory and its subdirectories.
4. **Generate SHA256 Hash**: The tool computes the SHA256 hash for each selected file.
5. **Compare with Trusted Hashes**: The tool compares each computed hash against the provided list of trusted SHA256 hashes. If the file's computed hash matches any of the trusted hashes, it is considered safe. If no match is found, the file might be compromised.

---

## Signature Verification with Cosign

You can verify the authenticity of the executables using `cosign`:

### PowerShell
```powershell
# Verify the .exe file
cosign verify-blob sha256-hash-checker.exe `
    --bundle sha256-hash-checker-v1.0.0-windows-exe.bundle `
    --certificate-oidc-issuer "https://github.com/login/oauth" `
    --certificate-identity "farhan.khondakar@gmail.com"

# Verify the .msi file
cosign verify-blob sha256-hash-checker_0.1.0_x64_en-US.msi `
    --bundle sha256-hash-checker-v1.0.0-windows-msi.bundle `
    --certificate-oidc-issuer "https://github.com/login/oauth" `
    --certificate-identity "farhan.khondakar@gmail.com"