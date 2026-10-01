#![allow(dead_code)]

use crate::engine::Player;

pub struct NavigationEngine;

impl NavigationEngine {
    pub fn seek_relative(player: &Player, seconds: f64) {
        player.seek_relative(seconds);
    }

    pub fn seek_percent(player: &Player, percent: f64) {
        let stats = player.stats();
        if stats.duration > 0.0 {
            let target = (percent.clamp(0.0, 100.0) / 100.0) * stats.duration;
            player.seek_absolute(target);
        }
    }

    pub fn seek_to_time(player: &Player, time_sec: f64) {
        player.seek_absolute(time_sec);
    }

    pub fn frame_next(player: &Player) {
        player.frame_step();
    }

    pub fn frame_prev(player: &Player) {
        player.frame_back_step();
    }

    pub fn next_chapter(player: &Player) {
        let stats = player.stats();
        let cur = stats.time_pos;
        for chap in &stats.chapters {
            if chap.time_pos > cur + 1.0 {
                player.seek_absolute(chap.time_pos);
                return;
            }
        }
    }

    pub fn prev_chapter(player: &Player) {
        let stats = player.stats();
        let cur = stats.time_pos;
        for chap in stats.chapters.iter().rev() {
            if chap.time_pos < cur - 1.5 {
                player.seek_absolute(chap.time_pos);
                return;
            }
        }
        player.seek_absolute(0.0);
    }
}
