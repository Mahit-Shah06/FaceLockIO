# FSRC — Face-based Secure Runtime Control

FSRC is a **local, real-time security system** that controls user input based on live face presence.

It allows or blocks **keyboard and mouse input** depending on whether the owner’s face is detected by the webcam — **without stopping any running processes**.

Everything runs **offline**, on the user’s own device.

---

## What FSRC Does

FSRC:
- Monitors the webcam
- Periodically checks for the owner’s face
- Allows or blocks user input (keyboard/mouse)
- Does **not** stop running processes  
  (videos, games, downloads keep running)
- Runs completely offline

---

## Core Principles

- **Safety first**  
  Never permanently lock the user out

- **Local only**  
  No cloud, no uploads, no tracking

- **Low resource usage**  
  Designed to run quietly in the background

- **Modular architecture**  
  Easy to extend and refactor

- **Linux-first**  
  Windows and Android planned later

---

## High-Level Architecture

FSRC is designed as a set of clear, isolated blocks:


