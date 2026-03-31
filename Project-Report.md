# PROJECT REPORT

## Title

Save The Ship

---

## 1. Problem Statement

This project focuses on building a simple autonomous agent that can survive in a dynamic and unpredictable environment.

The core question explored:

> How can an AI agent make fast, real-time decisions to avoid multiple moving threats and maximize survival time?

Instead of solving a predefined problem, I chose to simulate a scenario where conditions continuously change, forcing the system to react instantly rather than rely on precomputed solutions.

---

## 2. Motivation

I chose this problem because it combines:

* game development
* real-time systems
* we build an autonomus ai agent that plays the game

It also relates to real-world systems like:

* obstacle avoidance in robotics
* autonomous navigation
* systems that operate under uncertainty with limited computation time

Additionally, it’s a good way to explore how far simple mathematical rules can go in producing intelligent-looking behavior.

---

## 3. Approach

### Tech Stack

* Rust
* Raylib

### System Overview

The simulation consists of:

* A ship (controlled manually or by AI)
* Asteroids that spawn dynamically from screen edges
* A circular zone restricting movement

Each frame, the system:

1. Updates positions
2. Spawns new asteroids
3. Applies AI or player input (toggleable)
4. Checks collisions
5. Renders the scene

---

## 4. AI Design

The AI is based on a reactive steering model.

### Key Ideas

1. The agent only considers nearby asteroids

2. **Distance-Based Avoidance:** Closer asteroids exert stronger influence

3. The closest asteroid is treated as the most critical danger

4. **Aggregate Avoidance:** All nearby asteroids contribute to a combined avoidance vector

### Final Movement

The final velocity is a combination of:

* Strong avoidance (closest threat)
* General avoidance (all threats)
* Centering force

This produces smooth, continuous movement.

---

## 5. Design Decisions

### Reactive AI vs Pathfinding

Pathfinding algorithms were not used because:

* The environment changes every frame
* There is no stable graph structure
* Computation would be too expensive for real-time updates

Reactive steering provides constant-time decisions and adapts instantly.

---

### Circular Boundary

A circular constraint was chosen instead of a rectangle because:

* It avoids corner trapping
* It creates uniform movement constraints
* It simplifies boundary handling

---

### Difficulty Scaling

Difficulty increases over time through:

* Faster asteroid velocities
* Higher spawn rates

This ensures the system is tested under increasingly stressful conditions.

---

## 6. Challenges Faced

### Boundary Locking

The agent sometimes got stuck near the edge due to conflicting forces.

Solution:

* Clamp position inside the safe radius
* Add a center-directed bias force

---

### Unstable Movement

Multiple competing vectors caused jitter or erratic motion.

Solution:

* Normalize vectors
* Clamp velocity
* Separate strongest and aggregate influences

---

### Parameter Tuning

Balancing forces required careful tuning:

* Too much avoidance → chaotic motion
* Too much center bias → increased collisions

This required iterative testing and adjustment.

---

## 7. Results

The final system:

* Successfully avoids multiple moving obstacles
* Maintains stable and smooth motion
* Demonstrates emergent behavior without explicit planning

In many cases, the AI survives longer than manual control, showing that even simple heuristics can perform well in reactive environments.

---

## 8. Limitations

* No prediction of future asteroid positions
* No learning or adaptation
* Performance depends on manually tuned parameters

The system is reactive rather than intelligent in a deeper sense.

---

## 9. Future Improvements

* Add trajectory prediction
* Explore reinforcement learning
* Optimize using spatial partitioning
* Introduce scoring or evaluation metrics

---

## 10. Conclusion

This project demonstrates that relatively simple vector-based logic can produce effective real-time behavior in dynamic systems.

It highlights how combining multiple small heuristics can lead to emergent and practical solutions without requiring complex algorithms.

---
