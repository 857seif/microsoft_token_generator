# microsoft_token_generator

![Language](https://img.shields.io/badge/Language-Rust-orange.svg)
![Build](https://img.shields.io/badge/Build-3.03%20MB-brightgreen.svg)
![Platform](https://img.shields.io/badge/Platform-Windows%2010%2F11-blue.svg)
![Network](https://img.shields.io/badge/Dependencies-Mitmproxy%20%26%20Python-blueviolet.svg)

An advanced, ultra-lightweight, and lightning-fast **Automation Toolkit** engineered in Rust for seamless Microsoft Live License token interception and emulation loops. This tool simplifies complex proxy networking into a seamless single-click user experience.

---

## 🛠️ Prerequisite: Core Environment Setup

Before launching the toolkit executable, you **MUST** configure the runtime framework dependencies. 

### Step 1: Install Python & Mitmproxy Core
1. Run `NexusToolkitApp.exe` as **Administrator** (Required for Registry changes).
2. Navigate directly to the 4th tab: **`DEPENDENCIES MANAGER`**.
3. Click **`DOWNLOAD PYTHON EXECUTABLE`** and install Python 3.11+. *(Ensure you check "Add Python to PATH" during installation).*
4. Click **`DOWNLOAD MITMPROXY EXECUTABLE`** and complete the standard installation wizard.
5. Click **`INSTALL REQUIRED LIBRARIES (PIP)`** to fetch backend site-packages natively.
6. Click **`EXECUTE FULL ENVIRONMENT VERIFICATION`** to ensure everything displays `STABLE VERIFIED ✔`.

### Step 2: Critical Certificate Installation 🔐
To capture secure HTTPS traffic without encountering connection handshake failures, you must install the local proxy SSL certificate:
1. Turn on the proxy once by clicking **`INITIALIZE CAPTURE`** in the tool, then open your web browser.
2. Go to the URL: **`http://mitm.it/`**
3. Locate the **Windows** platform section and click **Show Instructions / Download**.
4. Double-click the downloaded certificate file (e.g., `mitmproxy-ca-cert.p12`).
5. Choose **Current User** -> Click Next.
6. When prompted for Certificate Store, select **"Place all certificates in the following store"**.
7. Click **Browse** and choose **`Trusted Root Certification Authorities`**.
8. Finish the import wizard and click **Yes** on the Windows security warning pop-up.

---

## 🚀 Step-by-Step Operational Blueprint

<div align="center">
  
| Interface Tab | Objective | Primary Actions |
| :--- | :--- | :--- |
| **1. GET TOKEN** | Extract Live Authorization Ticket | Set Storage Path ➡️ Click `INITIALIZE CAPTURE` ➡️ Launch Game ➡️ Capture `captured_token.txt` |
| **2. INJECT TOKEN** | Bypass Local Validation & Emulate Server | Load Dataset ➡️ Click `ENGAGE EMULATION SERVER` ➡️ Play Offline/Shared |

</div>
[Administrator Launch]


TARGET STORAGE PATH   ──► [Click BROWSE DIRECTORY] to assign output location


INITIALIZE CAPTURE    ──► Loops proxy engine on 127.0.0.1:8080 & opens Xbox/Store



Launch Targeted Game  ──► System automatically dumps payload to 'captured_token.txt'

TERMINATE PIPELINE  ──► Disengages proxy loop and normalizes global internet state

1. Log into the **Microsoft Store / Xbox App** using the account containing the legitimate game license or active Game Pass subscription.
2. Under **`TARGET STORAGE PATH`** at the bottom, use **`BROWSE DIRECTORY`** to pick where your token will save.
3. Head over to **`GET TOKEN`** tab and hit **`INITIALIZE CAPTURE`**.
4. The application will launch your Microsoft App hubs automatically. **Start the game**.
5. Once the game challenges the auth server, the backend catches the token structure and writes it to `captured_token.txt`.
6. Click **`TERMINATE PIPELINE`** to stop capturing and restore your network settings.

---

### 📤 Tab 2: How to Inject a Captured Token (Emulation Server)

---


BROWSE TOKEN MANUALLY   ──► Import a file shared by a friend



ENGAGE EMULATION SERVER  ──► Intercepts runtime challenges with injected matrices

1. Log into your **OWN personal Xbox Account** (The profile that does *not* own the game).
2. Navigate to the **`INJECT TOKEN`** tab.
3. Click **`AUTO-LOAD FROM EXPORT PATH`** (if you generated the token on the same machine) or click **`BROWSE TOKEN MANUALLY`** to select a friend's text file.
4. Verify your dataset status turns into *Staged Dataset Target*.
5. Hit **`ENGAGE EMULATION SERVER`**. The application will host local loops on port 8080 and route Microsoft authentication.
6. Launch your game and play online with your personalized gamer tag! 🎮
7. When done, click **`DISENGAGE SERVER`** to fully wipe background processes safely.

---

## 🛠️ Network Rescue Controls (Tab 3: Proxy Status)
If your application terminates unexpectedly or encounters a crash loop, your web connectivity might seem paused because of lingering registry variables. 
* Navigate to **`PROXY STATUS`**.
* Click **`QUERY LIVE REGISTRY STATUS`** to check structural gateway states.
* Click **`FORCE DISABLE SYSTEM PROXY`** to reset Windows network properties instantly.

---

## 🔒 Security Statement & Open Source Integrity
* **Zero Malicious Footprint:** Built natively with memory-safe **Rust**, preventing memory leaks or common PyInstaller heuristics detection signatures.
* **Open Source Framework:** All interception automation behaviors (`capture.py`/`inject.py`) are fully generated dynamic assets stored transparently in the workspace environment for inspection.

---
<align="center"><i>Developed with precision. Powered by Rust. Engineered for the community.</i></align>


## Usage

1. **Build the project:**

    ```sh
    cargo build --release
    ```

    The resulting binary will be located in `target/release/microsoft_token_generator.exe`.

   ## Disclaimer

This project is for educational and research purposes only. Use responsibly and respect software licenses.
