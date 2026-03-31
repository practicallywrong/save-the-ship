## Save The Ship 🚀

A simple survival simulation where a ship tries to avoid incoming asteroids for as long as possible.

---

## Overview

This project features:

* Manual control mode
* AI-controlled survival mode
* Increasing difficulty over time

The focus is on real-time decision-making rather than traditional gameplay mechanics.

---

## Features

* Smooth movement using vector math
* Dynamic asteroid spawning
* AI based on reactive steering
* Toggleable debug visualization

---

## Controls

* W A S D → Move (Manual mode)
* SPACE → Toggle AI
* H → Toggle ray visualization
* R → Restart

---

## Setup

```bash
git clone https://github.com/practicallywrong/save-the-ship
cd save-the-ship
cargo run
```

Make sure Raylib is installed before running.

---

## How the AI Works

The AI:

* Moves away from nearby asteroids
* Prioritizes the closest threat
* Applies a center bias to stay within bounds

The final motion is the result of combining these influences each frame.

---

## Purpose

This project was built to explore real-time AI behavior using simple mathematical models instead of complex algorithms.

---

## Future Work

* Smarter AI models
* Learning-based approaches
* Performance improvements

---
