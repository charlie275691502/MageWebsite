use mage_battle::game::Game;
use mage_battle::character::CharacterType;
use mage_battle::player::TeamId;
use mage_battle::web_server;

#[tokio::main]
async fn main() {
    // 檢查命令行參數
    let args: Vec<String> = std::env::args().collect();

    // 如果有 --web 參數，直接啟動Web服務器
    if args.len() > 1 && (args[1] == "--web" || args[1] == "-w") {
        println!("🚀 啟動 MageBattle Web 服務器...");
        println!("📍 服務器地址: http://localhost:3000");
        println!("⛔ 按 Ctrl+C 停止服務器\n");

        web_server::start_server().await;
        return;
    }

    // CLI模式
    println!("=== MageBattle ===");
    println!("歡迎來到法師對戰！");
    println!();
    println!("遊戲模式:");
    println!("  [1] 開始新遊戲");
    println!("  [2] Web服務器模式");
    println!();
    println!("提示: 也可以使用 'cargo run --release -- --web' 直接啟動Web服務器");

    let mode = get_input("請選擇 (1-2): ");

    match mode.trim() {
        "1" => {
            println!("\n開始新遊戲...");
            start_cli_game();
        }
        "2" => {
            println!("\n🚀 啟動 Web 服務器模式...");
            println!("📍 服務器地址: http://localhost:3000");
            println!("⛔ 按 Ctrl+C 停止服務器\n");

            web_server::start_server().await;
        }
        _ => {
            println!("無效選擇");
        }
    }
}

fn start_cli_game() {
    println!("\n選擇玩家角色:");
    println!("  [1] 火毒法師");
    println!("  [2] 木風法師");
    println!("  [3] 雷毒法師");
    println!("  [4] 水風法師");

    let characters = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::ThunderPoison,
        CharacterType::WaterWind,
    ];

    let mut player_names = Vec::new();
    let mut player_characters = Vec::new();

    for i in 0..4 {
        println!("\n玩家 {} (隊伍 {}):", i + 1, if i % 2 == 0 { "A" } else { "B" });

        let name = get_input(&format!("  輸入名字 (留空默認為玩家{}): ", i + 1));
        let name = if name.trim().is_empty() {
            format!("玩家{}", i + 1)
        } else {
            name.trim().to_string()
        };

        let choice = get_input("  選擇角色 (1-4): ");
        let char_idx = choice.trim().parse::<usize>().unwrap_or(1).saturating_sub(1).min(3);

        player_names.push(name);
        player_characters.push(characters[char_idx]);
    }

    println!("\n創建遊戲...");

    let mut game = Game::new(player_names, player_characters);

    println!("\n遊戲開始！");
    println!("隊伍A: {} & {}", game.players[0].name, game.players[2].name);
    println!("隊伍B: {} & {}", game.players[1].name, game.players[3].name);
    println!();

    // 遊戲主循環
    loop {
        if game.check_game_over() {
            break;
        }

        game.play_turn();

        let cont = get_input("\n繼續下一回合? (y/n): ");
        if cont.trim().to_lowercase() != "y" {
            break;
        }
    }

    if let Some(winner) = game.get_winner() {
        println!("\n🎉 遊戲結束！隊伍 {} 獲勝！", if winner == TeamId::Team0 { "A" } else { "B" });
    }
}

fn get_input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
