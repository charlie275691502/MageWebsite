import React, { useState } from "react";
import "./Guide.css";

interface GuideProps {
  isOpen: boolean;
  onClose: () => void;
}

type TabKey = "structure" | "stats" | "attributes" | "turns" | "cards" | "targeting" | "deck" | "special";

const Guide: React.FC<GuideProps> = ({ isOpen, onClose }) => {
  const [activeTab, setActiveTab] = useState<TabKey>("structure");

  if (!isOpen) return null;

  const handleBackgroundClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  const tabs: { key: TabKey; label: string }[] = [
    { key: "structure", label: "遊戲結構" },
    { key: "stats", label: "玩家狀態" },
    { key: "attributes", label: "屬性精通" },
    { key: "turns", label: "回合結構" },
    { key: "cards", label: "卡牌法術" },
    { key: "targeting", label: "目標規則" },
    { key: "deck", label: "牌庫管理" },
    { key: "special", label: "特殊規則" },
  ];

  const renderContent = () => {
    switch (activeTab) {
      case "structure":
        return (
          <div className="guide-content">
            <h2>🎮 遊戲結構</h2>

            <h3>玩家 & 隊伍</h3>
            <ul>
              <li><strong>總玩家數</strong>: 4 (玩家 0, 1, 2, 3)</li>
              <li><strong>隊伍</strong>: 2v2 格式
                <ul>
                  <li>隊伍 A: 玩家 0 & 玩家 2</li>
                  <li>隊伍 B: 玩家 1 & 玩家 3</li>
                </ul>
              </li>
              <li><strong>回合順序</strong>: 順序進行 (0 → 1 → 2 → 3 → 0...)</li>
              <li><strong>座位排列</strong>: 玩家圍成一圈 (P0 → P1 → P2 → P3 → 回到P0)</li>
            </ul>

            <h3>勝利條件</h3>
            <p>消滅對方隊伍的兩名玩家</p>
          </div>
        );

      case "stats":
        return (
          <div className="guide-content">
            <h2>❤️ 玩家狀態</h2>

            <h3>HP (生命值)</h3>
            <ul>
              <li><strong>初始</strong>: 50 HP</li>
              <li><strong>最大值</strong>: 50 HP (可以被治療超過最大值)</li>
              <li><strong>死亡</strong>: HP 降至 0 時發生</li>
            </ul>

            <h3>護盾</h3>
            <ul>
              <li><strong>初始</strong>: 0</li>
              <li><strong>最大值</strong>: 無限制</li>
              <li><strong>功能</strong>: 在 HP 之前吸收傷害</li>
              <li><strong>重要</strong>: 護盾不會吸收剩餘傷害（多餘的傷害會消失）</li>
              <li><strong>範例 1</strong>: 50 HP, 7 護盾, 受到 5 傷害 → 50 HP, 2 護盾</li>
              <li><strong>範例 2</strong>: 50 HP, 7 護盾, 受到 10 傷害 → 50 HP, 0 護盾 (不是 47 HP)</li>
              <li><strong>例外</strong>: 護盾無法阻擋直接傷害類型</li>
            </ul>

            <h3>死亡 & 復活</h3>
            <div className="guide-subsection">
              <h4>當 HP = 0:</h4>
              <ul>
                <li>玩家立即死亡</li>
                <li>手牌全部棄掉</li>
                <li>所有增益/減益效果移除</li>
                <li>死亡期間無法行動</li>
              </ul>

              <h4>死亡計數器:</h4>
              <p>追蹤死亡回合數（在玩家回合時遞增）</p>

              <h4>復活:</h4>
              <ul>
                <li>死亡 2 個回合後自動復活</li>
                <li>復活時恢復 50% HP (25 HP)</li>
              </ul>
            </div>
          </div>
        );

      case "attributes":
        return (
          <div className="guide-content">
            <h2>✨ 屬性精通</h2>

            <h3>屬性類型</h3>
            <div className="guide-subsection">
              <h4>主要屬性 (4):</h4>
              <ul>
                <li>🔥 火 (Fire)</li>
                <li>🌳 木 (Wood)</li>
                <li>⚡ 雷 (Thunder)</li>
                <li>💧 水 (Water)</li>
              </ul>

              <h4>次要屬性 (2):</h4>
              <ul>
                <li>🌪️ 風 (Wind)</li>
                <li>☠️ 毒 (Poison)</li>
              </ul>
            </div>

            <h3>屬性等級</h3>
            <ul>
              <li><strong>範圍</strong>: 0 到 5</li>
              <li><strong>分配</strong>: 每回合 1 點</li>
              <li><strong>最大值</strong>: 單一屬性不能超過 5 級</li>
              <li><strong>滿級上限</strong>: 如果所有屬性都達到 5 級，跳過屬性分配階段</li>
            </ul>

            <h3>屬性精通加成</h3>
            <div className="guide-subsection">
              <h4>🔥 火</h4>
              <ul>
                <li><strong>Lv3</strong>: 所有屬性彈 +1 傷害</li>
                <li><strong>Lv5</strong>: 回合開始時，對所有敵人造成 1 技能傷害（繞過木 Lv5 減免）</li>
              </ul>

              <h4>🌳 木</h4>
              <ul>
                <li><strong>Lv3</strong>: 使用木系法術時：
                  <ul>
                    <li>代價: -1 HP（直接傷害，完全繞過護盾）</li>
                    <li>收益: +1 護盾</li>
                  </ul>
                </li>
                <li><strong>Lv5</strong>: 自己和隊友受到的所有法術傷害 -1</li>
              </ul>

              <h4>⚡ 雷</h4>
              <ul>
                <li><strong>Lv3</strong>: 雷系法術 +1 傷害</li>
                <li><strong>Lv5</strong>: 雷系法術額外 +2 傷害（與 Lv3 疊加共 +3）</li>
              </ul>

              <h4>💧 水</h4>
              <ul>
                <li><strong>Lv3</strong>: 使用水系法術時，自己 +1 HP</li>
                <li><strong>Lv5</strong>: 使用水系法術時，隊友也 +1 HP</li>
              </ul>

              <h4>🌪️ 風</h4>
              <ul>
                <li><strong>Lv2</strong>: 被風系法術擊中的目標本回合無法治療或獲得護盾</li>
                <li><strong>Lv5</strong>: 風系法術可以指定任何玩家為目標（不受預設目標限制）</li>
              </ul>

              <h4>☠️ 毒</h4>
              <ul>
                <li><strong>Lv2</strong>: 被毒系法術擊中的目標獲得混亂減益（下回合先出牌再分配屬性）</li>
                <li><strong>Lv5</strong>: 被毒系法術擊中的目標獲得沉默減益（下回合只能使用屬性彈）</li>
              </ul>
            </div>
          </div>
        );

      case "turns":
        return (
          <div className="guide-content">
            <h2>⏱️ 回合結構</h2>
            <p>每個玩家的回合包含 5 個階段：</p>

            <h3>1. 回合開始階段</h3>
            <p>自動觸發的效果：</p>
            <ul>
              <li>死亡計數器遞增（如果死亡）</li>
              <li>復活檢查（如果死亡 2 回合）</li>
              <li>火 Lv5 效果（對所有敵人造成 1 傷害）</li>
              <li>生命汲取效果（如果適用）</li>
              <li>再生效果（如果適用）</li>
              <li>燃盡效果（消耗 1 火屬性點）</li>
            </ul>

            <h3>2. 分配屬性階段</h3>
            <ul>
              <li>當前玩家分配 1 屬性點</li>
              <li>可選擇：火、木、雷、水、風、毒</li>
              <li>不能超過 5 級</li>
              <li><strong>特殊</strong>: 如果被混亂，此階段在出牌階段之後進行</li>
            </ul>

            <h3>3. 出牌階段</h3>
            <p>當前玩家<strong>必須</strong>正好打出 1 張牌：</p>

            <div className="guide-subsection">
              <h4>選項 A: 施放法術</h4>
              <ul>
                <li>選擇卡牌的上面或下面</li>
                <li>必須滿足屬性需求</li>
                <li>根據法術選擇目標</li>
              </ul>

              <h4>選項 B: 使用屬性彈</h4>
              <ul>
                <li>棄掉任意一張牌</li>
                <li>使用你的某個屬性進行攻擊</li>
                <li>傷害 = 屬性等級</li>
                <li>目標 = 最遠的存活敵人（自動）</li>
              </ul>

              <h4>選項 C: 使用解放技能</h4>
              <ul>
                <li>必須滿足解放需求</li>
                <li>每場遊戲只能使用一次</li>
                <li>參見角色說明了解詳情</li>
              </ul>

              <p><strong>不能跳過此階段！</strong></p>
            </div>

            <h3>4. 抽牌階段</h3>
            <ul>
              <li>從牌庫抽 1 張牌</li>
              <li>如果牌庫為空，將棄牌堆洗回牌庫</li>
              <li>如果玩家已解放，額外抽 +1 張牌（共 2 張）</li>
            </ul>

            <h3>5. 回合結束階段</h3>
            <p>自動清理：</p>
            <ul>
              <li>增益/減益持續時間遞減</li>
              <li>過期的效果被移除</li>
              <li>下一位玩家的回合開始</li>
            </ul>
          </div>
        );

      case "cards":
        return (
          <div className="guide-content">
            <h2>🃏 卡牌法術</h2>

            <h3>卡牌結構</h3>
            <ul>
              <li>每張卡有 1 或 2 個法術面</li>
              <li><strong>上面法術</strong>: 總是存在</li>
              <li><strong>下面法術</strong>: 可選（某些卡只有單面）</li>
              <li>卡牌存儲在 spells.json 中</li>
            </ul>

            <h3>法術需求</h3>
            <ul>
              <li>格式範例: "火2木1" 表示需要火 Lv2 且 木 Lv1</li>
              <li>必須滿足所有需求才能施放</li>
              <li>需求在施放時檢查</li>
            </ul>

            <h3>法術效果</h3>
            <p>常見的法術效果：</p>
            <ul>
              <li><strong>傷害</strong>: 對目標造成 X 點傷害</li>
              <li><strong>治療</strong>: 恢復目標 X 點 HP</li>
              <li><strong>護盾</strong>: 給予目標 X 點護盾</li>
              <li><strong>增益/減益</strong>: 施加狀態效果</li>
              <li><strong>特殊</strong>: 各種獨特效果（見卡牌）</li>
            </ul>

            <h3>屬性彈</h3>
            <p>使用屬性的特殊攻擊：</p>
            <div className="guide-subsection">
              <h4>需求:</h4>
              <ul>
                <li>1 張任意牌（棄掉）</li>
                <li>屬性等級 ≥ 1</li>
              </ul>

              <h4>特性:</h4>
              <ul>
                <li><strong>傷害</strong>: 等於屬性等級</li>
                <li><strong>目標</strong>: 最遠的存活敵人（自動）</li>
                <li><strong>類型</strong>: 算作法術傷害</li>
                <li><strong>附魔</strong>: 被認為具有該屬性彈的屬性，可獲得精通加成</li>
              </ul>

              <h4>精通加成:</h4>
              <ul>
                <li>火 Lv3 對所有屬性彈 +1 傷害</li>
                <li>雷 Lv3/5 對雷屬性彈額外 +1/+3</li>
              </ul>

              <h4>範例:</h4>
              <div className="guide-example">
                <p>火等級 3, 雷等級 5</p>
                <ul>
                  <li>使用火彈 → 傷害 = 3（火等級）+ 1（火 Lv3 精通）= 4</li>
                  <li>使用雷彈 → 傷害 = 5（雷等級）+ 1（火 Lv3）+ 3（雷 Lv5）= 9</li>
                </ul>
              </div>
            </div>
          </div>
        );

      case "targeting":
        return (
          <div className="guide-content">
            <h2>🎯 目標規則</h2>

            <h3>預設目標</h3>
            <p>對於大多數法術（如果沒有另外指定）：</p>
            <ul>
              <li><strong>必須指定最遠的存活敵人</strong></li>
              <li><strong>不能指定</strong>: 已死亡的玩家</li>
            </ul>

            <h3>最遠存活敵人</h3>
            <p>定義：按回合順序前進，距離最大的存活敵人</p>

            <div className="guide-example">
              <h4>範例（對玩家 0）:</h4>
              <ul>
                <li>P1（距離 1）- 敵人</li>
                <li>P2（距離 2）- 隊友</li>
                <li>P3（距離 3）- 敵人</li>
                <li><strong>最遠存活敵人</strong> = P3</li>
              </ul>

              <h4>範例（對玩家 0，P3 已死）:</h4>
              <ul>
                <li>P1（距離 1）- 敵人</li>
                <li>P2（距離 2）- 隊友</li>
                <li>P3（距離 3）- 敵人（死亡）</li>
                <li><strong>最遠存活敵人</strong> = P1</li>
              </ul>
            </div>

            <h3>屬性彈目標</h3>
            <ul>
              <li><strong>總是指定</strong>最遠存活敵人</li>
              <li>不能手動選擇目標</li>
              <li>風 Lv5 不影響屬性彈目標</li>
            </ul>

            <h3>法術卡目標</h3>
            <p>根據法術規格而異：</p>
            <ul>
              <li><strong>預設</strong>: 最遠存活敵人</li>
              <li><strong>自己</strong>: 施法者</li>
              <li><strong>隊友</strong>: 施法者或隊友</li>
              <li><strong>敵人</strong>: 任何存活的敵人</li>
              <li><strong>全體</strong>: 多個目標</li>
              <li><strong>風 Lv5 覆蓋</strong>: 允許風系法術自由選擇目標</li>
            </ul>
          </div>
        );

      case "deck":
        return (
          <div className="guide-content">
            <h2>📚 牌庫管理</h2>

            <h3>開始設置</h3>
            <ul>
              <li><strong>牌庫大小</strong>: 共 60 張牌（共享）</li>
              <li><strong>起始手牌</strong>: 每位玩家 5 張牌</li>
            </ul>

            <h3>遊戲進行中</h3>
            <ul>
              <li><strong>抽牌</strong>: 每回合 1 張（如果已解放則 2 張）</li>
              <li><strong>棄牌堆</strong>: 使用過的卡牌放在這裡</li>
              <li><strong>重新洗牌</strong>: 當牌庫空時，將棄牌堆洗回牌庫</li>
            </ul>

            <h3>死亡 & 卡牌</h3>
            <ul>
              <li>當玩家死亡時，所有手牌被棄掉</li>
              <li>當玩家復活時，抽 5 張新牌</li>
            </ul>
          </div>
        );

      case "special":
        return (
          <div className="guide-content">
            <h2>⚡ 特殊規則</h2>

            <h3>精通疊加</h3>
            <p>多個精通加成以相加方式疊加</p>
            <div className="guide-example">
              <h4>範例:</h4>
              <ul>
                <li>法術需要火 & 雷</li>
                <li>施法者有火 Lv3 和 雷 Lv5</li>
                <li>基礎傷害: 10</li>
                <li>火 Lv3: +1</li>
                <li>雷 Lv3: +1</li>
                <li>雷 Lv5: +2</li>
                <li><strong>總計</strong>: 10 + 1 + 1 + 2 = 14 傷害</li>
              </ul>
            </div>

            <h3>法術附魔</h3>
            <p>法術根據其需求被「附魔」上屬性：</p>
            <ul>
              <li>範例: "火2木1" 法術被附魔火和木</li>
              <li>附魔決定哪些精通加成適用</li>
              <li>木 Lv3 代價會對任何木附魔法術觸發</li>
            </ul>

            <h3>解放技能</h3>
            <ul>
              <li>每個角色有 1 個獨特的解放技能</li>
              <li>需要特定的屬性等級</li>
              <li>每場遊戲只能使用一次</li>
              <li>替代「出牌階段」</li>
              <li>使用解放後，角色永久獲得 +1 抽牌</li>
            </ul>

            <h4>角色與解放技能：</h4>
            <ul>
              <li><strong>火毒法師</strong>: 燃盡（火傷害 +5，每回合消耗 1 火）</li>
              <li><strong>木風法師</strong>: 守護木雕（自己 + 隊友傷害 -4，持續 3 次）</li>
              <li><strong>雷毒法師</strong>: 連鎖閃電（對所有其他玩家 10 傷害）</li>
              <li><strong>水風法師</strong>: 颱風眼（再生 7 HP，持續 4 回合）</li>
            </ul>
          </div>
        );

      default:
        return <div>內容載入中...</div>;
    }
  };

  return (
    <div className="guide-overlay" onClick={handleBackgroundClick}>
      <div className="guide-modal">
        <div className="guide-header">
          <h1>📖 遊戲指南</h1>
          <button className="guide-close-btn" onClick={onClose}>
            ✕
          </button>
        </div>

        <div className="guide-tabs">
          {tabs.map((tab) => (
            <button
              key={tab.key}
              className={`guide-tab ${activeTab === tab.key ? "active" : ""}`}
              onClick={() => setActiveTab(tab.key)}
            >
              {tab.label}
            </button>
          ))}
        </div>

        <div className="guide-body">
          {renderContent()}
        </div>
      </div>
    </div>
  );
};

export default Guide;
