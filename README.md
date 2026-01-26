FSRC — Face-based Secure Runtime Control
What is FSRC?

FSRC is a local, real-time security system that:

Monitors the webcam

Checks for the owner’s face periodically

Allows or blocks user input (keyboard/mouse) based on face presence

Does NOT stop running processes (videos, games, downloads keep running)

Everything runs offline, on the user’s device.

Core Principles

Safety first: never permanently lock the user out

Local only: no cloud, no uploads

Low resource usage

Modular design (easy to extend)

Linux-first (Windows later)

High-Level Architecture

FSRC is split into blocks (major responsibilities):

FSRC
├── Main
├── Core Controller
├── Vision System
├── Security Gate
├── Input Controller
└── UI


Each block has one job.

Block Responsibilities
Main

Program entry point

Starts the Core Controller

Handles clean shutdown

Core Controller

Controls program lifecycle

Loads configuration

Routes messages between blocks

No vision, no input blocking logic

Vision System

Camera access

Face detection

Face verification (owner / not owner)

Enrollment (add/remove faces)

Gesture detection (future)

⚠️ Vision never blocks input directly.

Security Gate

Central decision maker

Maintains system state:

ALLOW

WARN

BLOCK

Applies grace time logic

Decides when input should be allowed or blocked

Only this block decides security.

Input Controller

Manages input devices (keyboard, mouse, etc.)

Blocks or allows input based on Security Gate decisions

Does not make decisions itself

UI

Shows system state

Allows configuration changes

Allows device exceptions (e.g. headphones allowed)

Sends user intent to Core Controller

UI never enforces security.

Development Phases

Core starts & exits cleanly

Camera access

Face detection

Face verification

Security Gate logic

Input blocking

UI

Optimization

Extras (gestures, startup, Android)

What FSRC is NOT

Not cloud-based

Not spyware

Not process killer

Not instant-lock on single failure

Status

🚧 Early development — architecture & foundation phase.
