use crate::game::computer::AttackStrategy;
use crate::game::GameBoard;
use crate::Position;

pub struct RandomAttackStrategy;

impl AttackStrategy for RandomAttackStrategy {
    fn calculate_best_attack(&mut self, enemy_board: &GameBoard) -> Position {
        self.get_random_position(enemy_board)
    }
}
