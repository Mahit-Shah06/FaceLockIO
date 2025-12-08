# FaceLockIO – System Architecture Document (Summarized + Detailed)

## 0. Project Overview
FaceLockIO is a cross-platform real-time face-verified input gate.  
Keyboard and mouse input remain active only when a trusted user’s face is detected.  
This system implements continuous authentication with minimal CPU usage.

---

## 1. Purpose & Scope

### 1.1 Vision
- Detect authorized user’s face at 15–30 FPS.
- Grant or revoke input access instantly.
- Run continuously in the background.
- Auto-start on login.
- Support Linux (X11/Wayland) and Windows.

### 1.2 Problems It Solves
- Prevents accidental or malicious device use when user looks away.
- Protects sensitive sessions in shared environments.
- Provides biometric continuous authentication.

### 1.3 Success Criteria
- Authorization latency < 150 ms.
- CPU usage < 25–30% of one core.
- Accurate recognition with low false positives.
- Stable 24/7 runtime.

### 1.4 Out of Scope
- OS login replacement.
- Cloud syncing of biometric data.
- Camera LED suppression (hardware-limited).
- Mobile integration (future).

---

## 2. Requirements

### 2.1 Functional Requirements
- Real-time face detection (MediaPipe BlazeFace).
- Embedding generation (ArcFace ONNX).
- Authorization state machine.
- Input blocking layer:
  - X11: XInput/XTest + evdev
  - Wayland: portals or fallback locking
  - Windows: Low-level hooks
- Encrypted storage of face templates.
- Fallback unlock (hotkey + Argon2id passphrase).
- GUI for settings & user enrollment (Qt6).

### 2.2 Non-functional Requirements
- High performance, low latency.
- Privacy: no cloud, no raw images stored.
- Cross-platform compatibility.
- Resilience to camera failure.
- Restart-safe daemon behavior.

---

## 3. System Overview

### 3.1 Modules
1. **Vision Engine**  
   - Camera capture (OpenCV)  
   - Face detection (MediaPipe)  
   - Embedding extraction (ArcFace + ONNX Runtime)

2. **Auth Core**  
   - Compare embeddings using L2/Cosine distance  
   - N-frame hysteresis for lock/unlock  

3. **Input Control Layer**  
   - OS-specific keyboard/mouse blocking  

4. **GUI (Qt6)**  
   - Enrollment wizard  
   - Settings panel  
   - Live status  

5. **Storage**  
   - Encrypted SQLite DB (AES-256-GCM)  
   - Templates + config + fallback hash (Argon2id)

6. **Daemon**  
   - Runs continuously  
   - Loads config  
   - Coordinates Vision & Input modules  

### 3.2 Data Flow
1. Camera → Vision Engine → embeddings  
2. Embeddings → Auth Core → AUTHORIZED/UNAUTHORIZED  
3. State → Input Layer → allow/block input  
4. GUI ↔ Daemon (IPC)  
5. Config stored in encrypted DB  

---

## 4. Tech Stack

### 4.1 Languages
- C++20 (core + ML + OS integration)
- Qt6 (GUI)

### 4.2 Libraries
- OpenCV (camera)
- MediaPipe face detection
- ONNX Runtime (ArcFace)
- SQLite (encrypted)
- libsodium / OpenSSL (AES-256 encryption)
- Argon2 (fallback password hashing)

### 4.3 OS APIs
- X11 XInput, XTest, evdev (Linux)
- Windows SetWindowsHookEx
- Wayland portals (best effort)

---

## 5. Data Models

### 5.1 User
| Field | Type |
|-------|------|
| user_id | UUID |
| display_name | string |
| enabled | bool |
| created_at | timestamp |

### 5.2 FaceTemplate
| Field | Type |
|--------|------|
| template_id | UUID |
| user_id | FK(User) |
| embedding | float[512], AES-encrypted |
| created_at | timestamp |

### 5.3 SystemConfig
- threshold  
- frames_to_unlock  
- frames_to_lock  
- camera_device  
- fps_limit  
- fallback_enabled  

### 5.4 FallbackConfig
- hotkey_combo  
- Argon2id_hash  
- unlock_duration  

---

## 6. Program Flow

### 6.1 Authorization Cycle
1. Capture new frame  
2. Detect face  
3. Extract embedding  
4. Compute distance with stored template  
5. If match persists N frames → AUTHORIZED  
6. If mismatch persists M frames → UNAUTHORIZED  
7. Input layer updates permissions  

### 6.2 Fallback Flow
- User triggers hotkey  
- Enter passphrase  
- Argon2id verification  
- Temporary unlock  

---

## 7. Implementation Plan (Summary)

### Part A – Setup
- Git repo  
- CMake skeleton  
- Folder structure  

### Part B – Core Modules
- CameraManager  
- FaceDetector  
- FaceEmbedder  
- AuthManager  

### Part C – OS Input Layer
- Linux (X11 + evdev)  
- Windows (LL hooks)  

### Part D – GUI
- Qt6 basic windows  

---

## 8. Risks
- Wayland input blocking limitations  
- Camera LED cannot be disabled  
- Performance issues on low-end systems  
- OS security changes breaking hooks  

---

## 9. Notes / Ideas
- Gesture support (MediaPipe Hands)  
- Liveness detection upgrades  
- GPU acceleration  

---

## 10. Dev Log
A chronological list of progress updates (to be filled later).

