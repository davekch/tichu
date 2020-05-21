use crate::combinations::Trick;
use crate::deck::{Card, Deck, SpecialKind};
use log::debug;

pub struct TichuGame {
    deck: Deck,
    // holds the hands that are meant for players after dealing, None as soon as a player takes theirs
    hands: [Option<Vec<Card>>; 4],
    pub current_player: usize,
    player_points: [i16; 4],
    finished: Vec<usize>, // contains indices of players that finished, in order
    tricks: Vec<Trick>, // tricks in the middle of the table
    passes: u8, // number of times that players have passed (at 3, player wins the round)
    // scores[i][0] is for team 0,2 and scores[i][1] is for team 1,3
    scores: Vec<Vec<i16>>,
}

impl TichuGame {
    pub fn new() -> TichuGame {
        TichuGame {
            deck: Deck::new(),
            hands: [None, None, None, None],
            current_player: 0,
            player_points: [0, 0, 0, 0],
            passes: 0,
            finished: Vec::new(),
            scores: vec![vec![0, 0]],
            tricks: Vec::new(),
        }
    }

    pub fn shuffle_and_deal(&mut self) {
        self.deck.shuffle();
        let hands = self.deck.deal();
        for i in 0..4 {
            self.hands[i] = Some(hands[i].to_vec());
        }
    }

    pub fn take_hand(&mut self, i: usize) -> Option<Vec<Card>> {
        self.hands[i].take()
    }

    pub fn pass(&mut self) {
        // call this if a player doesn't want to play
        self.passes += 1;
    }

    pub fn add_trick(&mut self, trick: Trick) {
        // players must make sure themselves that trick is valid
        self.passes = 0; // chain of passes is interrupted
        self.tricks.push(trick);
    }

    pub fn next(&mut self) -> RoundStatus {
        // move current player
        // if the latest trick is the dog, the current player shifts by 2
        let mut dog = Trick::new();
        dog.push(Card::special(SpecialKind::Dog));
        if self.tricks.len() == 1 && self.tricks[0] == dog {
            self.current_player = (self.current_player + 2) % 4;
        } else {
            self.current_player = (self.current_player + 1) % 4;
        }
        let mut status = RoundStatus::Continue;
        // while loop to pass all finished players + collect tricks if someone wins
        while self.finished.contains(&self.current_player) || self.passes == 3 {
            if self.passes == 3 {
                // if 3 players pass, the current player wins this round
                self.passes = 0;
                // collect all the points
                for trick in &self.tricks {
                    debug!("player {} wins and gets {} points", self.current_player, trick.points());
                    self.player_points[self.current_player] += trick.points();
                }
                self.tricks = Vec::new();
                status = RoundStatus::TrickWin;
            }
            // if the current player has no cards left, they "pass"
            if self.finished.contains(&self.current_player) {
                self.passes += 1;
                self.current_player = (self.current_player + 1) % 4;
            }
        }
        status
    }

    pub fn mark_finished(&mut self, player_index: usize) -> RoundStatus {
        self.finished.push(player_index);
        let mut gamestatus: RoundStatus;
        // if only one player is left, the round has ended
        if self.finished.len() == 3 {
            // figure out who finished last
            // self.finished only contains the first three finishers so the last one is
            // (0+1+2+3 = 6) - sum(self.finished)
            let last: usize = 6 - self.finished.iter().sum::<usize>();
            let mut points = vec![0, 0];
            // the team of the first finisher gets to keep their own points
            points[self.finished[0] % 2] += self.player_points[self.finished[0]];
            // team of first finisher gets the points of last finisher
            points[self.finished[0] % 2] += self.player_points[last];
            // team of second and third finisher keep their own points
            points[self.finished[1] % 2] += self.player_points[self.finished[1]];
            points[self.finished[2] % 2] += self.player_points[self.finished[2]];
            // left cards of the last player go to opposing team. the value of the left cards is 100 - sum(points)
            points[(last + 1) % 2] += 100 - points.iter().sum::<i16>();
            // save and reset
            self.scores.push(points);
            self.player_points = [0, 0, 0, 0];
            gamestatus = RoundStatus::FinishRound;
        } else if self.finished.len() == 2 && (self.finished[0] % 2 == self.finished[1] % 2) {
            let mut points = vec![0, 0];
            points[self.finished[0] % 2] += 200;
            self.scores.push(points);
            self.player_points = [0, 0, 0, 0];
            gamestatus = RoundStatus::FinishRound;
        } else {
            gamestatus = RoundStatus::Continue;
        }
        // check if game is over
        if self.scores[self.scores.len() - 1][0] > 999 {
            gamestatus = RoundStatus::Team1Wins;
        } else if self.scores[self.scores.len() - 1][1] > 999 {
            gamestatus = RoundStatus::Team2Wins;
        }
        return gamestatus;
    }

    pub fn get_current_trick(&self) -> Option<&Trick> {
        if self.tricks.len() > 0 {
            Some(&self.tricks[self.tricks.len() - 1])
        } else {
            None
        }
    }

    pub fn get_current_score(&self) -> (i16, i16) {
        let points = &self.scores[self.scores.len() - 1];
        (points[0], points[1])
    }
}

#[derive(PartialEq, Eq)]
pub enum RoundStatus {
    Continue,
    TrickWin, // someone's won a trick but the round continues
    FinishRound, // round's finished
    Team1Wins,
    Team2Wins,
}
