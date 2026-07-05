//! The Runtime Cat — a living mascot with a state machine.
//! The cat is NOT Pandora. The box (logo) is Pandora.
//! The cat is the runtime spirit: mischievous, curious, alive.
//!
//! Physics rules:
//! - Weight, momentum, jump arcs, falling
//! - Idle breathing, paw movement, tail movement
//! - No cartoon teleportation

use std::time::Instant;

/// Possible cat behaviors.
#[derive(Debug, Clone, PartialEq)]
pub enum CatState {
    Idle,
    Sleeping,
    Stretching,
    Cleaning,
    Watching,
    SittingOnPrompt,
    FollowingCursor,
    Running,
    Yawning,
    Rolling,
    Walking,
    Sliding,
    Climbing,
    Scared,
    Hiding,
    CurledUp,
    Playing,
    Staring,
}

impl CatState {
    pub fn description(&self) -> &'static str {
        match self {
            CatState::Idle => "idle — watching the runtime",
            CatState::Sleeping => "sleeping — curled up on completed tasks",
            CatState::Stretching => "stretching — warming up for work",
            CatState::Cleaning => "cleaning — tidying up loose variables",
            CatState::Watching => "watching — observing the execution graph",
            CatState::SittingOnPrompt => "sitting on the prompt — refusing to move",
            CatState::FollowingCursor => "following the cursor — curious",
            CatState::Running => "running — chasing execution nodes",
            CatState::Yawning => "yawning — another successful build",
            CatState::Rolling => "rolling over — showing belly for attention",
            CatState::Walking => "walking across panels — exploring",
            CatState::Sliding => "sliding — jumped and landed smoothly",
            CatState::Climbing => "climbing — scaling window borders",
            CatState::Scared => "scared — compiler error detected",
            CatState::Hiding => "hiding — behind a widget",
            CatState::CurledUp => "curled up — napping on GPU usage",
            CatState::Playing => "playing — batting at loading indicators",
            CatState::Staring => "staring — directly at you",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            CatState::Idle => "🐱",
            CatState::Sleeping => "😴",
            CatState::Stretching => "🙆",
            CatState::Cleaning => "👅",
            CatState::Watching => "👀",
            CatState::SittingOnPrompt => "📌",
            CatState::FollowingCursor => "👉",
            CatState::Running => "🏃",
            CatState::Yawning => "🥱",
            CatState::Rolling => "🌀",
            CatState::Walking => "🚶",
            CatState::Sliding => "⬇️",
            CatState::Climbing => "🧗",
            CatState::Scared => "😱",
            CatState::Hiding => "🙈",
            CatState::CurledUp => "💤",
            CatState::Playing => "🧶",
            CatState::Staring => "👁️",
        }
    }
}

/// ASCII cat face variants for each state.
pub fn cat_face(state: &CatState) -> &'static str {
    match state {
        CatState::Idle => "  /\\_/\\\n ( o.o )\n  > ^ <",
        CatState::Sleeping => "  /\\_/\\\n ( -.- )\n  > ^ <\n z Z z",
        CatState::Stretching => "  /\\_/\\\n ( >o< )\n  > ^ <\n  ||",
        CatState::Cleaning => "  /\\_/\\\n ( . . )\n  > ^ <\n  ~~",
        CatState::Watching => "  /\\_/\\\n ( @.@ )\n  > ^ <",
        CatState::SittingOnPrompt => "  /\\_/\\\n ( -_- )\n  > ^ <",
        CatState::FollowingCursor => "  /\\_/\\\n ( O.o )\n  > ^ <",
        CatState::Running => "  /\\_/\\\n ( o.O )\n  > ^ <\n  !!",
        CatState::Yawning => "  /\\_/\\\n ( O )\n  > ^ <",
        CatState::Rolling => "  /\\_/\\\n ( °.° )\n  > ^ <\n  ~~~",
        CatState::Walking => "  /\\_/\\\n ( - )\n  > ^ <\n  ~~",
        CatState::Sliding => "  /\\_/\\\n ( °o° )\n  > ^ <\n  ==]",
        CatState::Climbing => "  /\\_/\\\n ( o.o )\n  > ^ <\n   |",
        CatState::Scared => "  /\\_/\\\n ( º.º )\n  > ^ <\n  !!!",
        CatState::Hiding => "  /\\_/\\\n ( .. )\n  > ^ <\n  ▌",
        CatState::CurledUp => "  /\\_/\\\n ( -.- )\n  > ^ <\n  ###",
        CatState::Playing => "  /\\_/\\\n ( @o@ )\n  > ^ <\n  ()",
        CatState::Staring => "  /\\_/\\\n ( •.• )\n  > ^ <",
    }
}

/// The cat's physical position on screen.
#[derive(Debug, Clone)]
pub struct CatPosition {
    pub x: i16,
    pub y: i16,
    pub target_x: i16,
    pub target_y: i16,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

/// The runtime cat mascot.
pub struct RuntimeCat {
    pub state: CatState,
    pub position: CatPosition,
    pub width: u16,
    pub height: u16,
    last_state_change: Instant,
    pub visible: bool,
    pub animations_enabled: bool,
}

impl RuntimeCat {
    pub fn new() -> Self {
        Self {
            state: CatState::Idle,
            position: CatPosition {
                x: 2,
                y: 1,
                target_x: 2,
                target_y: 1,
                velocity_x: 0.0,
                velocity_y: 0.0,
            },
            width: 10,
            height: 4,
            last_state_change: Instant::now(),
            visible: true,
            animations_enabled: true,
        }
    }

    /// Update the cat's state and physics. Call once per frame.
    pub fn update(&mut self, time_since_start: f32) {
        if !self.animations_enabled {
            return;
        }

        // State transitions based on time
        let elapsed = self.last_state_change.elapsed().as_secs_f32();

        // Change state every 5-15 seconds
        if elapsed > 5.0 + (time_since_start % 10.0) {
            self.transition_randomly();
            self.last_state_change = Instant::now();
        }

        // Physics: smooth movement toward target
        self.position.velocity_x += (self.position.target_x - self.position.x) as f32 * 0.01;
        self.position.velocity_y += (self.position.target_y - self.position.y) as f32 * 0.01;

        // Damping
        self.position.velocity_x *= 0.9;
        self.position.velocity_y *= 0.9;

        self.position.x += self.position.velocity_x as i16;
        self.position.y += self.position.velocity_y as i16;

        // Clamp to reasonable bounds
        self.position.x = self.position.x.clamp(0, 80);
        self.position.y = self.position.y.clamp(0, 30);
    }

    fn transition_randomly(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let states = [
            CatState::Idle,
            CatState::Sleeping,
            CatState::Stretching,
            CatState::Cleaning,
            CatState::Watching,
            CatState::Yawning,
            CatState::Walking,
            CatState::Playing,
            CatState::Staring,
        ];
        let next = states[rng.gen_range(0..states.len())].clone();
        self.state = next;

        // Random target position
        self.position.target_x = rng.gen_range(0..70);
        self.position.target_y = rng.gen_range(0..25);
    }

    /// React to a runtime event. Returns true if the cat reacted.
    pub fn react_to_event(&mut self, event_type: &str) -> bool {
        match event_type {
            e if e.contains("error") || e.contains("failure") => {
                self.state = CatState::Scared;
                self.last_state_change = Instant::now();
                true
            }
            e if e.contains("success") || e.contains("complete") => {
                self.state = CatState::Playing;
                self.last_state_change = Instant::now();
                true
            }
            e if e.contains("build") || e.contains("compile") => {
                self.state = CatState::Watching;
                self.last_state_change = Instant::now();
                true
            }
            _ => false,
        }
    }

    /// Render the cat as a string.
    pub fn render(&self) -> String {
        if !self.visible {
            return String::new();
        }
        let face = cat_face(&self.state);
        let status = format!("[{}] {}", self.state.emoji(), self.state.description());
        format!("{}\n{}", face, status)
    }
}

impl Default for RuntimeCat {
    fn default() -> Self {
        Self::new()
    }
}
