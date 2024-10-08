use anchor_lang::prelude::*;

declare_id!("ERZeBKsnZ19HqiRCJ4npkVkwf2hvUKN5GSsDW5Uy84bm");

#[program]
pub mod feedingdanny {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>, player_name: String) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.player = Player {
            name: player_name,
            size: 1,
            score: 0,
            exp: 0,
            level: 1,
        };
        game.fish = Vec::new();
        game.leaderboard = Vec::new();
        Ok(())
    }

    pub fn spawn_fish(ctx: Context<SpawnFish>, size: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let fish = Fish { size };
        game.fish.push(fish);
        Ok(())
    }

    pub fn eat_fish(ctx: Context<EatFish>, fish_index: u64) -> Result<()> {
        let game = &mut ctx.accounts.game;
        
        if fish_index >= game.fish.len() as u64 {
            return Err(ErrorCode::InvalidIndex.into());
        }
    
        let fish_size = game.fish[fish_index as usize].size;
        
        if game.player.size > fish_size {
            let exp_gain = fish_size as u32 * 10 + 10;
            game.player.exp += exp_gain;
            game.player.score += fish_size as u32;
            
            while game.player.exp >= (game.player.level * 100).into() {
                game.player.level += 1;
                game.player.size += 1;
            }
    
            game.fish.remove(fish_index as usize);
    
            // Clone necessary data before updating leaderboard
            let player_name = game.player.name.clone();
            let player_score = game.player.score;
    
            // Update leaderboard
            update_leaderboard(game, player_name, player_score);
        } else {
            return Err(ErrorCode::FishTooLarge.into());
        }
    
        Ok(())
    }

    pub fn get_player_stats(ctx: Context<GetPlayerStats>) -> Result<PlayerStats> {
        let game = &ctx.accounts.game;
        Ok(PlayerStats {
            name: game.player.name.clone(),
            size: game.player.size,
            score: game.player.score,
            exp: game.player.exp,
            level: game.player.level,
        })
    }

    pub fn get_leaderboard(ctx: Context<GetLeaderboard>) -> Result<Vec<LeaderboardEntry>> {
        Ok(ctx.accounts.game.leaderboard.clone())
    }
}

fn update_leaderboard(game: &mut Game, player_name: String, score: u32) {
    if let Some(entry) = game.leaderboard.iter_mut().find(|e| e.name == player_name) {
        entry.score = score.max(entry.score);
    } else {
        game.leaderboard.push(LeaderboardEntry { name: player_name, score });
    }
    game.leaderboard.sort_by(|a, b| b.score.cmp(&a.score));
}

// The rest of the code remains the same...

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 1000)] // Adjust space as needed
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SpawnFish<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
}

#[derive(Accounts)]
pub struct EatFish<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
}

#[derive(Accounts)]
pub struct GetPlayerStats<'info> {
    pub game: Account<'info, Game>,
}

#[derive(Accounts)]
pub struct GetLeaderboard<'info> {
    pub game: Account<'info, Game>,
}

#[account]
pub struct Game {
    pub player: Player,
    pub fish: Vec<Fish>,
    pub leaderboard: Vec<LeaderboardEntry>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Player {
    pub name: String,
    pub size: u8,
    pub score: u32,
    pub exp: u32,
    pub level: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Fish {
    pub size: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PlayerStats {
    pub name: String,
    pub size: u8,
    pub score: u32,
    pub exp: u32,
    pub level: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct LeaderboardEntry {
    pub name: String,
    pub score: u32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid fish index")]
    InvalidIndex,
    #[msg("Fish is too large to eat")]
    FishTooLarge,
}