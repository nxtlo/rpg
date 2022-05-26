// BSD 3-Clause License

// Copyright (c) 2022, nxtlo
// All rights reserved.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:

// 1. Redistributions of source code must retain the above copyright notice, this
// list of conditions and the following disclaimer.

// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.

// 3. Neither the name of the copyright holder nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::fmt;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Stats {
    pub mp5: u32,
    pub hp5: u32,
    pub health: u32,
    pub evasion: u32,
    pub movement_speed: u32,
    pub attack_speed: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            mp5: 0,
            hp5: 0,
            health: 0,
            evasion: 0,
            movement_speed: 0,
            attack_speed: 0,
        }
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            concat!(
                "Stats(",
                "MP5: {} ",
                "HP5: {} ",
                "Health: {} ",
                "Evasion: {} ",
                "Movement Speed: {} ",
                "Attack Speed: {})",
            ),
            self.mp5, self.hp5, self.health, self.evasion, self.movement_speed, self.attack_speed
        )
    }
}

impl Stats {}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Resistense {
    toxcin: u32,
    elemental: u32,
    void: u32,
    radiant: u32,
}

impl fmt::Display for Resistense {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            concat!(
                "Resistense(",
                "Toxin: {} ",
                "Elemental: {} ",
                "Void: {} ",
                "Radiant: {})",
            ),
            self.toxcin, self.elemental, self.void, self.radiant
        )
    }
}

impl Default for Resistense {
    fn default() -> Self {
        Resistense {
            toxcin: 0,
            elemental: 0,
            void: 0,
            radiant: 0,
        }
    }
}

impl Resistense {}
